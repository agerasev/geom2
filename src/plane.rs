use crate::{Closed, Line, impl_approx_eq};
use glam::Vec2;

/// A half-plane defined by a boundary line.
///
/// The half-plane is the set of points on one side of a line.
/// The boundary line is defined by a normal vector and an offset.
/// The normal points from inside to outside.
///
/// ```text
///              ^
///    outside   | normal
///              |
/// -------------*------- boundary line
/// //////////// ^ //////////
/// // inside // | offset ///
/// //////////// | //////////
/// //////////// + (0,0) ////
/// ```
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HalfPlane {
    /// Normal of the half-plane edge, pointing from inside to outside.
    pub normal: Vec2,
    /// Minimal distance from the origin to the half-plane edge.
    ///
    /// If the origin is inside the half-plane then it is positive,
    /// when origin is outside — it is negative.
    pub offset: f32,
}

impl HalfPlane {
    /// Create a half-plane from a point on the boundary and a normal vector.
    ///
    /// The normal must be normalized and points from inside to outside.
    pub fn from_normal(point: Vec2, normal: Vec2) -> Self {
        Self {
            normal,
            offset: point.dot(normal),
        }
    }

    /// Construct from two points lying on edge.
    ///
    /// When looking from the first point to the second one,
    /// then the left side is inside the half-plane while the right side is outside.
    pub fn from_edge(Line(a, b): Line) -> Self {
        Self::from_normal(a, -(b - a).perp().normalize())
    }

    /// Minimal distance to the edge from the `point`.
    /// It is positive if `point` is outside of the half-plane, and negative if inside.
    pub fn distance(&self, point: Vec2) -> f32 {
        point.dot(self.normal) - self.offset
    }

    /// Get some point on the edge.
    pub fn boundary_point(&self) -> Vec2 {
        self.normal * self.offset
    }

    /// Get the boundary line of this half-plane.
    pub fn edge(&self) -> Line {
        let p = self.boundary_point();
        Line(p - 0.5 * self.normal.perp(), p + 0.5 * self.normal.perp())
    }
}

impl Closed for HalfPlane {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        -self.distance(point).signum() as i32
    }
}

impl_approx_eq!(HalfPlane, f32, normal, offset);
