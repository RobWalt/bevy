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
    commands.spawn(Camera2dBundle::default());
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
                let offset = Vec2::Y * n as f32 * 10.0;
                let start = Vec2::X * 100.0 + offset - Vec2::ONE * 50.0 - Vec2::X * 200.0;
                let end = Vec2::Y * 100.0 + offset - Vec2::ONE * 50.0 - Vec2::X * 200.0;
                gizmos
                    .animated_line_2d(start, end, color)
                    .segments(n as usize)
                    .speed(speed);
                let center = (start + end) / 2.0;
                gizmos
                    .animated_arc_2d(
                        center + Vec2::X * 250.0,
                        0.0,
                        180.0_f32.to_radians(),
                        50.0,
                        color,
                    )
                    .segments(n as usize)
                    .speed(speed)
                    .resolution(n as u32);
                gizmos.line_2d(
                    center - Vec2::Y * 50.0 + Vec2::X * 250.0,
                    center + Vec2::Y * 50.0 + Vec2::X * 250.0,
                    color,
                );
            });
    }
}
