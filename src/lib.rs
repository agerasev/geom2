#![no_std]

mod circle;
mod line;
mod plane;
mod polygon;
mod util;

pub use self::{
    circle::{Arc, ArcVertex, Circle, CircleSegment},
    line::{Line, LineSegment},
    plane::HalfPlane,
    polygon::{Edge, Polygon, Vertex},
    util::{AsIterator, AsMap},
};

use core::f32;
use glam::Vec2;

pub const EPS: f32 = 1e-8;

/// Shape that has an oriented edge.
pub trait Bounded {
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

/// Intersection of two figures
pub trait Intersect<T: Intersect<Self, Output = Self::Output> + ?Sized> {
    type Output: Sized;
    fn intersect(&self, other: &T) -> Option<Self::Output>;
}

/// Insrsection of two figures where resulting figure type can be selected.
pub trait IntersectTo<T: IntersectTo<Self, U> + ?Sized, U> {
    fn intersect_to(&self, other: &T) -> Option<U>;
}

impl<U: Intersect<V, Output = W>, V: Intersect<U, Output = W>, W> IntersectTo<V, W> for U {
    fn intersect_to(&self, other: &V) -> Option<W> {
        self.intersect(other)
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

impl Moment {
    pub fn merge(self, other: Self) -> Self {
        let area = self.area + other.area;
        if area.abs() < EPS {
            return Self::default();
        }
        let centroid = (self.centroid * self.area + other.centroid * other.area) / area;
        Self { area, centroid }
    }
}
