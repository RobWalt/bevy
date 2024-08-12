//! This example demonstrates Bevy's immediate mode animated drawing API intended for visual debugging.

use bevy::{color::palettes::css::*, input::mouse::MouseWheel, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_example_collection)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 7.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..Default::default()
    });
}

fn draw_example_collection(
    mut gizmos: AnimatedGizmos,
    mut max_gizmos: Local<isize>,
    mut ev_scroll: EventReader<MouseWheel>,
) {
    if !ev_scroll.is_empty() {
        *max_gizmos = (*max_gizmos + ev_scroll.read().map(|e| e.y as isize).sum::<isize>()).max(0);
    }
    let colors = [
        RED, ORANGE, YELLOW, DARK_GREEN, GREEN, LIGHT_BLUE, LIGHT_CYAN, AZURE, BLUE, VIOLET,
    ];
    if *max_gizmos >= 0 {
        (1..=*max_gizmos + 1)
            .zip(colors.into_iter().cycle())
            .for_each(|(n, color)| {
                let speed = n as f32 * 0.01;
                let offset = Vec3::Y * n as f32 * 0.1;
                let start = Vec3::X + offset - Vec3::ONE * 0.5 - Vec3::Y * 2.0;
                let end = Vec3::Y + offset - Vec3::ONE * 0.5 - Vec3::Y * 2.0;
                gizmos
                    .animated_line(start, end, color)
                    .segments(n as usize)
                    .speed(speed);
                let center = (start + end) / 2.0;
                let offset = Vec3::X;
                gizmos
                    .animated_arc_long(end + offset * 2.0, start, center + offset, color)
                    .segments(n as usize)
                    .speed(speed)
                    .resolution(n as u32);
            });
    }
}
