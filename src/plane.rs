use crate::{Closed, Line};
use glam::Vec2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HalfPlane {
    /// Normal of the half-plane edge, pointing from inside to outside.
    pub normal: Vec2,
    /// Minimal distance from the origin to the half-plane edge.
    ///
    /// If the origin is inside the half-plane then it is positive, when origin is outside — it is negative.
    pub offset: f32,
}

impl HalfPlane {
    /// Normal must be normalized.
    pub fn from_normal(point: Vec2, normal: Vec2) -> Self {
        Self {
            normal,
            offset: point.dot(normal),
        }
    }

    /// Construct from two points lying on edge.
    ///
    /// When looking from the first point to the second one, then the left side is inside the half-plane while the right side is outside.
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

    pub fn edge(&self) -> Line {
        let p = self.boundary_point();
        Line(p, p + self.normal.perp())
    }
}

impl Closed for HalfPlane {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        -self.distance(point).signum() as i32
    }
}
