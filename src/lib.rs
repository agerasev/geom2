//! # geom2 - 2D Geometry Primitives and Operations
//!
//! A Rust library for 2D computational geometry with `no_std` support.
//!
//! ## Overview
//!
//! `geom2` provides a comprehensive set of 2D geometric primitives and operations
//! for computational geometry applications. The library is designed with performance
//! and correctness in mind, featuring robust handling of edge cases and
//! floating-point precision issues.
//!
//! ## Key Features
//!
//! - **`no_std` compatible** - Works in embedded and constrained environments
//! - **Comprehensive primitives** - Lines, circles, arcs, polygons, half-planes
//! - **Geometric operations** - Intersection, containment, area calculation, winding numbers
//! - **Robust floating-point handling** - EPS-based tolerance for numerical stability
//! - **Generic design** - Flexible vertex and edge types with iterator-based APIs
//! - **Approximation support** - Optional `approx` feature for approximate equality comparisons
//!
//! ## Basic Usage
//!
//! ```rust
//! use geom2::{Line, LineSegment, Circle, Disk, Polygon, Closed, Integrable, Intersect};
//! use glam::Vec2;
//!
//! // Create geometric primitives
//! let line = Line(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
//! let disk = Disk::new(Vec2::new(0.0, 0.0), 1.0);
//!
//! // Check point containment
//! assert!(disk.contains(Vec2::new(0.5, 0.5)));
//!
//! // Compute geometric properties
//! let area = disk.area(); // π
//! let centroid = disk.centroid(); // Vec2::new(0.0, 0.0)
//!
//! // Intersection operations
//! let circle = Circle {
//!     center: Vec2::new(0.0, 0.0),
//!     radius: 1.0,
//! };
//! let intersections = circle.intersect(&line);
//! ```
//!
//! ## Numerical Precision
//!
//! The library uses a global `EPS` constant (`1e-8`) for floating-point comparisons.
//! All geometric operations are designed to handle edge cases and numerical
//! instability within this tolerance.
//!
//! ## Features
//!
//! - **`approx`** - Enables approximate equality comparisons using the `approx` crate.
//!   When enabled, geometric types implement `approx::AbsDiffEq` and `approx::RelativeEq`.
//!
//! ## Design Philosophy
//!
//! 1. **Correctness First** - Robust handling of degenerate cases and floating-point precision
//! 2. **Performance** - Efficient algorithms with minimal allocations
//! 3. **Flexibility** - Generic types that work with various storage backends
//! 4. **`no_std` Compatibility** - Suitable for embedded and constrained environments

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

/// Global epsilon value for floating-point comparisons.
///
/// This constant (`1e-8`) is used throughout the library for tolerance-based
/// comparisons to handle numerical instability in geometric computations.
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

/// A shape that has computable geometric moments (area, centroid).
pub trait Integrable {
    /// Compute the moment of the shape.
    ///
    /// The moment includes area (zeroth moment) and centroid (first moment).
    fn moment(&self) -> Moment;

    /// Area of the shape.
    fn area(&self) -> f32 {
        self.moment().area
    }

    /// Centroid (center of mass) of the shape.
    fn centroid(&self) -> Vec2 {
        self.moment().centroid
    }
}

/// Intersection of two figures
pub trait Intersect<T: Intersect<Self, Output = Self::Output> + ?Sized> {
    type Output: Sized;
    fn intersect(&self, other: &T) -> Option<Self::Output>;
}

/// Intersection of two figures where resulting figure type can be selected.
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
