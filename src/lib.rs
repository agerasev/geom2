#![no_std]

mod arc;
mod circle;
mod line;
mod plane;
mod polygon;
mod util;

#[cfg(test)]
mod tests;

pub(crate) use self::util::approx::impl_approx_eq;
pub use self::{
    arc::{Arc, ArcVertex, DiskSegment},
    circle::{Circle, Disk},
    line::{Line, LineSegment},
    plane::HalfPlane,
    polygon::{Edge, Polygon, Vertex, circle::ArcPolygon},
    util::{AsIterator, AsMap},
};

use core::f32;
use either::Either;
use glam::Vec2;

pub const EPS: f32 = 1e-8;

/// Shape that has an (oriented) edge.
pub trait Closed {
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

pub trait Integrable {
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

impl_approx_eq!(Moment, f32, area, centroid);

impl<T: Closed> Closed for Option<T> {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        match self {
            Some(shape) => shape.winding_number_2(point),
            None => 0,
        }
    }
}

impl<T: Integrable> Integrable for Option<T> {
    fn moment(&self) -> Moment {
        match self {
            Some(shape) => shape.moment(),
            None => Moment::default(),
        }
    }
}

impl<L: Closed, R: Closed> Closed for Either<L, R> {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        match self {
            Either::Left(left) => left.winding_number_2(point),
            Either::Right(right) => right.winding_number_2(point),
        }
    }
}

impl<L: Integrable, R: Integrable> Integrable for Either<L, R> {
    fn moment(&self) -> Moment {
        match self {
            Either::Left(left) => left.moment(),
            Either::Right(right) => right.moment(),
        }
    }
}
