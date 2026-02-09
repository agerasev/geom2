#![no_std]

mod circle;
mod line;
mod plane;
mod polygon;

pub use self::{
    circle::{Arc, ArcVertex, Circle, CircleOrSegment, CircleSegment},
    line::{Line, LineSegment},
    plane::HalfPlane,
    polygon::{Edge, Polygon, Vertex},
};

use core::f32;
use glam::Vec2;

pub const EPS: f32 = 1e-8;

/// Shape that has an oriented edge.
pub trait Bound {
    // fn bounding_box(&self) -> (Vec2, Vec2);

    /// The angle of edge rotation around point divided by PI.
    ///
    /// E.g. a non-self-intersecting counterclockwise polygon returns `2`
    /// for points inside of it, and `0` for points outside.
    ///
    /// Result is unspecified within boundary [`EPS`]-neighbourhood.
    fn winding_number_2(&self, point: Vec2) -> i32;

    /// Check that the `point` is inside the shape.
    fn contains(&self, point: Vec2) -> bool {
        self.winding_number_2(point) > 0
    }
}

pub trait Integrate {
    /// Moment of the shape
    fn moment(&self) -> Moment;

    fn area(&self) -> f32 {
        self.moment().area
    }
    fn centroid(&self) -> Vec2 {
        self.moment().centroid
    }
}

/// Moment of the shape
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Moment {
    /// Zeroth moment
    pub area: f32,
    /// First moment
    pub centroid: Vec2,
}

pub trait Intersect<T: Intersect<Self, U> + ?Sized, U> {
    /// Abstract intersection of two figures.
    fn intersect(&self, other: &T) -> Option<U>;
}
