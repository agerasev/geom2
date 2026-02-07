#![no_std]

mod circle;
mod line;
mod plane;
mod polygon;

pub use self::{
    circle::Circle,
    line::{Line, LineSegment},
    plane::HalfPlane,
    polygon::Polygon,
};

use core::f32;
use glam::Vec2;

pub const EPS: f32 = 1e-8;

/// Specific geometric shape.
pub trait Shape {
    // fn bounding_box(&self) -> (Vec2, Vec2);

    /// Check that the `point` is inside the shape.
    ///
    /// Shape is considered to be closed rather than open.
    /// That means the boundary points is inside the shape.
    fn is_inside(&self, point: Vec2) -> bool;

    /// Moments of the shape
    fn moments(&self) -> Moments;
    fn area(&self) -> f32 {
        self.moments().area
    }
    fn centroid(&self) -> Vec2 {
        self.moments().centroid
    }
}

/// Moments of the shape
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Moments {
    /// Zeroth moment
    pub area: f32,
    /// First moment
    pub centroid: Vec2,
}

pub trait Intersect<T: Intersect<Self, U> + ?Sized, U> {
    /// Abstract intersection of two figures.
    fn intersect(&self, other: &T) -> Option<U>;
}
