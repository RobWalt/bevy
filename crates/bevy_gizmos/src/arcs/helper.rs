use std::f32::consts::{FRAC_PI_2, TAU};

use bevy_math::{Quat, Vec2, Vec3};

use crate::circles::DEFAULT_CIRCLE_RESOLUTION;

pub(crate) fn arc_2d_inner(
    direction_angle: f32,
    arc_angle: f32,
    radius: f32,
    resolution: u32,
) -> impl Iterator<Item = Vec2> {
    let start = direction_angle - arc_angle / 2.0;
    (0..resolution + 1)
        .map(move |n| n as f32 / resolution as f32)
        .map(move |percentage| percentage * arc_angle)
        .map(move |offset| start + FRAC_PI_2 + offset)
        .map(|angle| angle.sin_cos())
        .map(|(sin, cos)| Vec2::new(cos, sin))
        .map(move |vec2| vec2 * radius)
}

pub(crate) fn arc_3d_inner(
    start_vertex: Vec3,
    center: Vec3,
    rotation: Quat,
    angle: f32,
    radius: f32,
    resolution: u32,
) -> impl Iterator<Item = Vec3> {
    // drawing arcs bigger than TAU degrees or smaller than -TAU degrees makes no sense since
    // we won't see the overlap and we would just decrease the level of details since the resolution
    // would be larger
    let angle = angle.clamp(-TAU, TAU);
    (0..=resolution)
        .map(move |frac| frac as f32 / resolution as f32)
        .map(move |percentage| angle * percentage)
        .map(move |frac_angle| Quat::from_axis_angle(Vec3::Y, frac_angle) * start_vertex)
        .map(move |p| rotation * (p * radius) + center)
}

// helper function for getting a default value for the resolution parameter
pub(crate) fn resolution_from_angle(angle: f32) -> u32 {
    ((angle.abs() / TAU) * DEFAULT_CIRCLE_RESOLUTION as f32).ceil() as u32
}

pub(crate) fn angle_inverted(angle: f32) -> f32 {
    if angle > 0.0 {
        TAU - angle
    } else if angle < 0.0 {
        -TAU - angle
    } else {
        0.0
    }
}

pub(crate) fn from_to_param_converter(
    center: Vec3,
    from: Vec3,
    to: Vec3,
    angle_fn: impl Fn(f32) -> f32,
) -> (Vec3, Quat, f32, f32) {
    // `from` and `to` can be the same here since in either case nothing gets rendered and the
    // orientation ambiguity of `up` doesn't matter
    let from_axis = (from - center).normalize_or_zero();
    let to_axis = (to - center).normalize_or_zero();
    let (up, angle) = Quat::from_rotation_arc(from_axis, to_axis).to_axis_angle();

    let angle = angle_fn(angle);
    let radius = center.distance(from);
    let rotation = Quat::from_rotation_arc(Vec3::Y, up);

    let start_vertex = rotation.inverse() * from_axis;

    (start_vertex, rotation, angle, radius)
}
