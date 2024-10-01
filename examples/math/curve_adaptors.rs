//! This example demonstrates all of the [`Curve`] adaptor methods with the help of gizmos.
use bevy::color::palettes::css::*;
use bevy::prelude::*;

#[path = "../helpers/camera_controller.rs"]
mod camera_controller;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(camera_controller::CameraControllerPlugin);

    app.init_state::<state::SelectedAdaptor>();

    app.init_gizmo_group::<curve::ThinGizmos>()
        .add_systems(Startup, curve::init_thin_gizmos);

    app.add_systems(Startup, (camera::spawn_camera, text::setup_text));

    app.add_systems(Update, (curve::render_grid, curve::render_curves));

    app.run();
}

/// module which is responsible for rendering the curves
pub mod curve {
    use ops::FloatPow;

    use super::*;

    /// Helper gizmos group for thiner lines
    #[derive(Default, Reflect, GizmoConfigGroup)]
    pub struct ThinGizmos;

    /// initialize thin helper gizmo group
    pub fn init_thin_gizmos(mut configs: ResMut<GizmoConfigStore>) {
        configs.config_mut::<ThinGizmos>().0.line_width = 0.5;
    }

    /// render a grid for better orientation in the scene
    pub fn render_grid(mut gizmos: Gizmos<ThinGizmos>) {
        gizmos.grid(
            Isometry3d::new(Vec3::NEG_Y * 3.0, Quat::from_rotation_arc(Vec3::Z, Vec3::Y)),
            UVec2::ONE * 50,
            Vec2::ONE,
            WHITE,
        );
    }

    /// render a basic curve
    pub fn render_curves(mut gizmos: Gizmos, time: Res<Time>) {
        let rotation_around_y = Quat::from_axis_angle(Vec3::Y, time.elapsed_seconds());
        let start = rotation_around_y * Vec3::ZERO;
        let end = rotation_around_y * Vec3::X;
        let curve = FunctionCurve::new(Interval::new(-1.0, 1.0).unwrap(), |t| {
            start.lerp(end, t) + Vec3::Y * t.cubed()
        });
        let slow_time = time.elapsed_seconds() * 0.25;
        let sample_points = (0..=100)
            .map(|p| p as f32 / 100.0)
            .map(|p| curve.domain().start() + curve.domain().length() * p);
        let ball_time = if curve.domain().is_bounded() {
            let ball_time_curve = easing::LinearCurve::new(
                curve.domain().start() - 0.25,
                curve.domain().end() + 0.25,
            );
            let time_for_ball = slow_time.rem_euclid(curve.domain().length());
            ball_time_curve.sample_clamped(time_for_ball)
        } else {
            let inf_factor = 100_000.0;
            let ball_time_curve = easing::LinearCurve::new(-inf_factor, inf_factor);
            let time_for_ball = slow_time / (inf_factor * 2.0);
            ball_time_curve.sample_clamped(time_for_ball)
        };
        gizmos.linestrip(curve.sample_iter_clamped(sample_points), VIOLET);
        if let Some(ball_position) = curve.sample(ball_time) {
            gizmos.sphere(Isometry3d::from_translation(ball_position), 0.1, ORANGE);
        }
    }
}

/// helper module for dealing with camera
pub mod camera {
    use super::*;

    /// sets up a simple camera with the camera controller
    pub fn spawn_camera(mut commands: Commands) {
        commands.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 3.0, 6.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            camera_controller::CameraController::default(),
        ));
    }
}

/// helper module for state and state transition
pub mod state {
    use super::*;

    /// State for tracking which primitives are currently displayed
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default, Reflect)]
    #[allow(missing_docs)]
    pub enum SelectedAdaptor {
        /// No adaptor
        #[default]
        Nothing,
        Reverse,
        Repeat,
        Forever,
        PingPong,
        Chain,
        Continuation,
    }
}

/// helper module for text
pub mod text {
    use super::*;

    /// Marker component for header text
    #[derive(Debug, Clone, Component, Default, Reflect)]
    pub struct HeaderText;

    /// Marker component for header node
    #[derive(Debug, Clone, Component, Default, Reflect)]
    pub struct HeaderNode;

    /// system which initializes ui text
    pub fn setup_text(mut commands: Commands) {
        let text = format!("{text:?}", text = state::SelectedAdaptor::default());
        let style = TextStyle::default();
        let instructions = "Press 'Up' or 'Down' to switch to the next/previous adaptor";
        let text = ["Adaptor: ", text.as_str(), "\n\n", instructions]
            .map(|text| TextSection::new(text, style.clone()));

        commands
            .spawn((
                HeaderNode,
                NodeBundle {
                    style: Style {
                        justify_self: JustifySelf::Center,
                        top: Val::Px(5.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    HeaderText,
                    TextBundle::from_sections(text).with_text_justify(JustifyText::Center),
                ));
            });
    }
}
