//! A module for the [`AnimatedGizmos`] [`SystemParam`].
//!
//! These are gizmos which render non-static line segments. Instead the segments are moving along
//! the line in a primitive based direction. The rules for the direction of the animation are
//! listed below
//!
//! - lines: from `start` to `end`
//  when implemented:
//  - primitices: counter clockwise when the primitive's normal is pointing towards the camera

use std::{
    f32::consts::TAU,
    ops::{Deref, DerefMut},
};

use bevy_color::Color;
use bevy_ecs::system::{Res, SystemParam};
use bevy_math::{Isometry3d, Quat, Vec2, Vec3};
use bevy_time::Time;

use crate::{
    arcs::helper::from_to_param_converter,
    prelude::{DefaultGizmoConfigGroup, GizmoConfigGroup, Gizmos},
};

/// A [`SystemParam`] for drawing animated gizmos, whose segments cycle along the line in a
/// primitive based direction. The rules for the direction of the animation are listed below
///
/// - lines: from `start` to `end`
///
/// This is essentially a utility wrapper for [`Gizmos`]. For additional information about the
/// [`Gizmos`] themselves, please refer to the linked documentation.
///
/// The wrapper additionally queries `Res<Time<T>>` through
/// [`SystemParam`] which can be used to visuallize changes over
/// time.
///
/// The type of time `T` can be defined using a generic type parameter, theoretically allowing
/// animations to occur in [`Fixed`](bevy_time::Fixed) time schedules. By default, animated gizmos
/// utilize the standard [`Time<()>`](bevy_time::Time). More information about this can be found
/// under [`Time`].
///
/// It's important to note that you can still use it to draw standard, non-animated shapes just
/// like with regular [`Gizmos`]. This means you don't need to add both [`AnimatedGizmos`] and
/// [`Gizmos`] params to your systems.
///
/// # Example
/// ```
/// # use bevy_gizmos::prelude::*;
/// # use bevy_render::prelude::*;
/// # use bevy_math::prelude::*;
/// # use bevy_color::palettes::basic::GREEN;
/// fn system(mut gizmos: AnimatedGizmos) {
///     // animated gizmos method
///     gizmos.animated_line(Vec3::ZERO, Vec3::X, GREEN)
///           .segments(10)
///           .speed(0.5);
///     // regular gizmos method
///     gizmos.line(Vec3::ZERO, Vec3::NEG_X, GREEN);
/// }
/// # bevy_ecs::system::assert_is_system(system);
/// ```
#[derive(SystemParam)]
pub struct AnimatedGizmos<'w, 's, Config = DefaultGizmoConfigGroup, Clear = (), TimeKind = ()>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    gizmos: Gizmos<'w, 's, Config, Clear>,
    time: Res<'w, Time<TimeKind>>,
}

impl<'w, 's, Config, Clear, TimeKind> Deref for AnimatedGizmos<'w, 's, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    type Target = Gizmos<'w, 's, Config, Clear>;
    fn deref(&self) -> &Self::Target {
        &self.gizmos
    }
}

impl<'w, 's, Config, Clear, TimeKind> DerefMut for AnimatedGizmos<'w, 's, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.gizmos
    }
}

/// A builder returned by [`AnimatedGizmos::animated_line`].
pub struct AnimatedLineBuilder<'a, 'w, 's, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    gizmos: &'a mut AnimatedGizmos<'w, 's, Config, Clear, TimeKind>,
    // start position of the animated line
    start: Vec3,
    // end position of the animated line
    end: Vec3,
    // color of the animated line
    color: Color,

    // number of segments of the animated line
    segments: usize,
    // speed factor for the animation
    speed: f32,
}

impl<Config, Clear, TimeKind> AnimatedLineBuilder<'_, '_, '_, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    /// Sets the number of animated segments that make up the line.
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }

    /// Sets the animation speed factor for the line.
    ///
    /// This determines the velocity at which the line segments move from the starting point to the endpoint.
    pub fn speed(mut self, factor: f32) -> Self {
        self.speed = factor;
        self
    }
}

impl<'w, 's, Config, Clear, TimeKind> AnimatedGizmos<'w, 's, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    /// Draw an animated line in 3D from `start` to `end`.
    ///
    /// The line is split into segments that move from `start` to `end` over time. This emphasizes
    /// the direction of the line and provides a clearer sense of its length.
    ///
    /// This should be called for each frame the line needs to be rendered.
    ///
    /// # Example
    /// ```
    /// # use bevy_gizmos::prelude::*;
    /// # use bevy_render::prelude::*;
    /// # use bevy_math::prelude::*;
    /// # use bevy_color::palettes::basic::GREEN;
    /// fn system(mut gizmos: AnimatedGizmos) {
    ///     gizmos.animated_line(Vec3::ZERO, Vec3::X, GREEN)
    ///           .segments(10)
    ///           .speed(0.5);
    /// }
    /// # bevy_ecs::system::assert_is_system(system);
    /// ```
    #[inline]
    pub fn animated_line(
        &mut self,
        start: Vec3,
        end: Vec3,
        color: impl Into<Color>,
    ) -> AnimatedLineBuilder<'_, 'w, 's, Config, Clear, TimeKind> {
        AnimatedLineBuilder {
            gizmos: self,
            start,
            end,
            color: color.into(),

            segments: 5,
            speed: 0.1,
        }
    }
}

impl<Config, Clear, TimeKind> Drop for AnimatedLineBuilder<'_, '_, '_, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    fn drop(&mut self) {
        if !self.gizmos.gizmos.enabled {
            return;
        }

        // prevent division by zero, we always want to have at least one segment
        if self.segments == 0 {
            return;
        }

        let diff = self.end - self.start;
        let color = self.color;
        sample_percentages(
            self.segments,
            self.gizmos.time.elapsed_seconds(),
            self.speed,
        )
        .map(|percentages| percentages.map(|percent| self.start + percent * diff))
        .for_each(|[start, end]| {
            self.gizmos.line(start, end, color);
        });
    }
}

/// A builder returned by [`AnimatedGizmos::animated_line_2d`].
pub struct AnimatedLine2dBuilder<'a, 'w, 's, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    gizmos: &'a mut AnimatedGizmos<'w, 's, Config, Clear, TimeKind>,
    // start position of the animated line
    start: Vec2,
    // end position of the animated line
    end: Vec2,
    // color of the animated line
    color: Color,

    // number of segments of the animated line
    segments: usize,
    // speed factor for the animation
    speed: f32,
}

impl<Config, Clear, TimeKind> AnimatedLine2dBuilder<'_, '_, '_, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    /// Sets the number of animated segments that make up the line.
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }

    /// Sets the animation speed factor for the line.
    ///
    /// This determines the velocity at which the line segments move from the starting point to the endpoint.
    pub fn speed(mut self, factor: f32) -> Self {
        self.speed = factor;
        self
    }
}

impl<'w, 's, Config, Clear, TimeKind> AnimatedGizmos<'w, 's, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    /// Draw an animated line in 2D from `start` to `end`.
    ///
    /// The line is split into segments that move from `start` to `end` over time. This emphasizes
    /// the direction of the line and provides a clearer sense of its length.
    ///
    /// This should be called for each frame the line needs to be rendered.
    ///
    /// # Example
    /// ```
    /// # use bevy_gizmos::prelude::*;
    /// # use bevy_render::prelude::*;
    /// # use bevy_math::prelude::*;
    /// # use bevy_color::palettes::basic::GREEN;
    /// fn system(mut gizmos: AnimatedGizmos) {
    ///     gizmos.animated_line_2d(Vec2::ZERO, Vec2::X, GREEN)
    ///           .segments(10)
    ///           .speed(0.5);
    /// }
    /// # bevy_ecs::system::assert_is_system(system);
    /// ```
    #[inline]
    pub fn animated_line_2d(
        &mut self,
        start: Vec2,
        end: Vec2,
        color: impl Into<Color>,
    ) -> AnimatedLine2dBuilder<'_, 'w, 's, Config, Clear, TimeKind> {
        AnimatedLine2dBuilder {
            gizmos: self,
            start,
            end,
            color: color.into(),

            segments: 5,
            speed: 0.1,
        }
    }
}

impl<Config, Clear, TimeKind> Drop for AnimatedLine2dBuilder<'_, '_, '_, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    fn drop(&mut self) {
        if !self.gizmos.gizmos.enabled {
            return;
        }

        // prevent division by zero, we always want to have at least one segment
        if self.segments == 0 {
            return;
        }

        let diff = self.end - self.start;
        let color = self.color;
        sample_percentages(
            self.segments,
            self.gizmos.time.elapsed_seconds(),
            self.speed,
        )
        .map(|percentages| percentages.map(|percent| self.start + percent * diff))
        .for_each(|[start, end]| {
            self.gizmos.line_2d(start, end, color);
        });
    }
}

/// A builder returned by [`AnimatedGizmos::animated_arc_long`] & [`AnimatedGizmos::animated_arc_short`].
pub struct AnimatedArcBuilder<'a, 'w, 's, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    gizmos: &'a mut AnimatedGizmos<'w, 's, Config, Clear, TimeKind>,
    // from position of the animated arc
    from: Vec3,
    // end position of the animated arc
    to: Vec3,
    // center position of the animated arc
    center: Vec3,
    // color of the animated line
    color: Color,

    angle_fn: fn(f32) -> f32,

    // number of segments of the animated line
    segments: usize,
    // speed factor for the animation
    speed: f32,
    // detail of the arc segments
    resolution: Option<u32>,
}

impl<Config, Clear, TimeKind> AnimatedArcBuilder<'_, '_, '_, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    /// Sets the number of animated segments that make up the arc.
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }

    /// Sets the animation speed factor for the arc.
    ///
    /// This determines the velocity at which the arc segments move from the starting point to the endpoint.
    pub fn speed(mut self, factor: f32) -> Self {
        self.speed = factor;
        self
    }

    /// Set the number of lines used to approximate the geometry of this arc.
    pub fn resolution(mut self, resolution: u32) -> Self {
        self.resolution.replace(resolution);
        self
    }
}

impl<'w, 's, Config, Clear, TimeKind> AnimatedGizmos<'w, 's, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    /// Draw an animated arc in 3D from `from` to `to` with the given `center`. The arc runs over
    /// the longer of the two paths from `from` to `to`.
    ///
    /// The arc is split into segments that move from `from` to `to` over time. This emphasizes
    /// the direction of the arc and provides a clearer sense of its length.
    ///
    /// This should be called for each frame the arc needs to be rendered.
    ///
    /// # Example
    /// ```
    /// # use bevy_gizmos::prelude::*;
    /// # use bevy_render::prelude::*;
    /// # use bevy_math::prelude::*;
    /// # use bevy_color::palettes::basic::GREEN;
    /// fn system(mut gizmos: AnimatedGizmos) {
    ///     gizmos.animated_arc_long(Vec3::X, Vec3::Y, Vec3::ZERO, GREEN)
    ///           .segments(10)
    ///           .speed(0.5);
    /// }
    /// # bevy_ecs::system::assert_is_system(system);
    /// ```
    #[inline]
    pub fn animated_arc_long(
        &mut self,
        from: Vec3,
        to: Vec3,
        center: Vec3,
        color: impl Into<Color>,
    ) -> AnimatedArcBuilder<'_, 'w, 's, Config, Clear, TimeKind> {
        fn angle_fn_long(angle: f32) -> f32 {
            if angle > 0.0 {
                TAU - angle
            } else if angle < 0.0 {
                -TAU - angle
            } else {
                0.0
            }
        }
        AnimatedArcBuilder {
            gizmos: self,
            from,
            to,
            center,
            color: color.into(),

            angle_fn: angle_fn_long,

            segments: 5,
            speed: 0.1,
            resolution: None,
        }
    }

    /// Draw an animated arc in 3D from `from` to `to` with the given `center`. The arc runs over
    /// the shorter of the two paths from `from` to `to`.
    ///
    /// The arc is split into segments that move from `from` to `to` over time. This emphasizes
    /// the direction of the arc and provides a clearer sense of its length.
    ///
    /// This should be called for each frame the arc needs to be rendered.
    ///
    /// # Example
    /// ```
    /// # use bevy_gizmos::prelude::*;
    /// # use bevy_render::prelude::*;
    /// # use bevy_math::prelude::*;
    /// # use bevy_color::palettes::basic::GREEN;
    /// fn system(mut gizmos: AnimatedGizmos) {
    ///     gizmos.animated_arc_short(Vec3::X, Vec3::Y, Vec3::ZERO, GREEN)
    ///           .segments(10)
    ///           .speed(0.5);
    /// }
    /// # bevy_ecs::system::assert_is_system(system);
    /// ```
    #[inline]
    pub fn animated_arc_short(
        &mut self,
        from: Vec3,
        to: Vec3,
        center: Vec3,
        color: impl Into<Color>,
    ) -> AnimatedArcBuilder<'_, 'w, 's, Config, Clear, TimeKind> {
        AnimatedArcBuilder {
            gizmos: self,
            from,
            to,
            center,
            color: color.into(),

            angle_fn: std::convert::identity,

            segments: 5,
            speed: 0.1,
            resolution: None,
        }
    }
}

impl<Config, Clear, TimeKind> Drop for AnimatedArcBuilder<'_, '_, '_, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    fn drop(&mut self) {
        if !self.gizmos.gizmos.enabled {
            return;
        }

        // prevent division by zero, we always want to have at least one segment
        if self.segments == 0 {
            return;
        }

        let (start_vertex, rotation, angle, radius) =
            from_to_param_converter(self.center, self.from, self.to, self.angle_fn);
        let angle = angle.clamp(-TAU, TAU);
        let isometry = Isometry3d::new(self.center, rotation);
        let color = self.color;

        sample_percentages(
            self.segments,
            self.gizmos.time.elapsed_seconds(),
            self.speed,
        )
        .map(|percentages| {
            percentages
                .map(|percent| percent * angle)
                .map(|frac_angle| Quat::from_axis_angle(Vec3::Y, frac_angle) * start_vertex)
                .map(|vec3| isometry * (vec3 * radius))
        })
        .for_each(|[start, end]| {
            let arc_builder = self
                .gizmos
                .short_arc_3d_between(self.center, start, end, color);
            if let Some(resolution) = self.resolution {
                arc_builder.resolution(resolution);
            }
        });
    }
}

/// A builder returned by [`AnimatedGizmos::animated_arc_2d`].
pub struct AnimatedArc2dBuilder<'a, 'w, 's, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    gizmos: &'a mut AnimatedGizmos<'w, 's, Config, Clear, TimeKind>,
    position: Vec2,
    direction_angle: f32,
    arc_angle: f32,
    radius: f32,
    color: Color,

    // number of segments of the animated line
    segments: usize,
    // speed factor for the animation
    speed: f32,
    // detail of the arc segments
    resolution: Option<u32>,
}

impl<Config, Clear, TimeKind> AnimatedArc2dBuilder<'_, '_, '_, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    /// Sets the number of animated segments that make up the arc.
    pub fn segments(mut self, segments: usize) -> Self {
        self.segments = segments;
        self
    }

    /// Sets the animation speed factor for the arc.
    ///
    /// This determines the velocity at which the arc segments move from the starting point to the endpoint.
    pub fn speed(mut self, factor: f32) -> Self {
        self.speed = factor;
        self
    }

    /// Set the number of lines used to approximate the geometry of this arc.
    pub fn resolution(mut self, resolution: u32) -> Self {
        self.resolution.replace(resolution);
        self
    }
}

impl<'w, 's, Config, Clear, TimeKind> AnimatedGizmos<'w, 's, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    /// Draw an arc, which is a part of the circumference of a circle, in 2D.
    ///
    /// The arc is split into segments that move from `from` to `to` over time. This emphasizes
    /// the direction of the arc and provides a clearer sense of its length.
    ///
    /// This should be called for each frame the arc needs to be rendered.
    ///
    /// # Arguments
    /// - `position` sets the center of this circle.
    /// - `direction_angle` sets the counter-clockwise  angle in radians between `Vec2::Y` and
    ///     the vector from `position` to the midpoint of the arc.
    /// - `arc_angle` sets the length of this arc, in radians.
    /// - `radius` controls the distance from `position` to this arc, and thus its curvature.
    /// - `color` sets the color to draw the arc.
    ///
    /// # Example
    /// ```
    /// # use bevy_gizmos::prelude::*;
    /// # use bevy_render::prelude::*;
    /// # use bevy_math::prelude::*;
    /// # use std::f32::consts::PI;
    /// # use bevy_color::palettes::basic::{GREEN, RED};
    /// fn system(mut gizmos: AnimatedGizmos) {
    ///     gizmos.animated_arc_2d(Vec2::ZERO, 0., PI / 4., 1., GREEN);
    ///
    ///     // Arcs have 32 line-segments by default.
    ///     // You may want to increase this for larger arcs.
    ///     gizmos
    ///         .animated_arc_2d(Vec2::ZERO, 0., PI / 4., 5., RED)
    ///         .resolution(64);
    /// }
    /// # bevy_ecs::system::assert_is_system(system);
    /// ```
    #[inline]
    pub fn animated_arc_2d(
        &mut self,
        position: Vec2,
        direction_angle: f32,
        arc_angle: f32,
        radius: f32,
        color: impl Into<Color>,
    ) -> AnimatedArc2dBuilder<'_, 'w, 's, Config, Clear, TimeKind> {
        AnimatedArc2dBuilder {
            gizmos: self,
            position,
            direction_angle,
            arc_angle,
            radius,
            color: color.into(),

            segments: 5,
            speed: 0.1,
            resolution: None,
        }
    }
}

impl<Config, Clear, TimeKind> Drop for AnimatedArc2dBuilder<'_, '_, '_, Config, Clear, TimeKind>
where
    Config: GizmoConfigGroup,
    Clear: 'static + Send + Sync,
    TimeKind: Default + 'static + Send + Sync,
{
    fn drop(&mut self) {
        if !self.gizmos.gizmos.enabled {
            return;
        }

        // prevent division by zero, we always want to have at least one segment
        if self.segments == 0 {
            return;
        }

        let color = self.color;

        sample_percentages(
            self.segments,
            self.gizmos.time.elapsed_seconds(),
            self.speed,
        )
        .map(|[start, end]| ([start, end], (start - end).abs()))
        .map(|([start, _end], length)| {
            (
                self.direction_angle + start * self.arc_angle + length,
                length * self.arc_angle,
            )
        })
        .for_each(|(direction_angle, arc_angle)| {
            let arc_builder = self.gizmos.arc_2d(
                self.position,
                direction_angle,
                arc_angle,
                self.radius,
                color,
            );
            if let Some(resolution) = self.resolution {
                arc_builder.resolution(resolution);
            }
        });
    }
}

fn sample_percentages(
    num_segments: usize,
    delta_t: f32,
    speed: f32,
) -> impl Iterator<Item = [f32; 2]> {
    let num_f32 = num_segments as f32;
    (0..=num_segments)
        .map(move |frac| frac as f32 / num_f32)
        .map(move |percent| percent + delta_t * speed)
        .map(move |percent| {
            // range 0.0..=(N+1)/N
            // -> line transitions out of visible range smoothly
            let modulo = 1.0 + num_f32.recip();
            percent % modulo
        })
        .map(move |percent| {
            let segment_length = (num_f32 * 2.0).recip();
            // range (-1/N)..=(N+1)/N
            // -> line transitions into visible range smoothly
            [(percent - segment_length), percent].map(|percent| percent.clamp(0.0, 1.0))
        })
}
