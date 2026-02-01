use crate::{Clump, Location, Shape};
use glam::Vec2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HalfPlane {
    /// Normal of the half-plane edge (pointing from occuped space to free space).
    pub normal: Vec2,
    /// Signed distance from the origin to the edge of the half-plane.
    ///
    /// If the origin is inside then it is positive, when origin is outside then it is negative.
    pub offset: f32,
}

impl HalfPlane {
    /// Normal must be normalized.
    pub fn from_normal(point: Vec2, normal: Vec2) -> Self {
        Self {
            normal,
            offset: -point.dot(normal),
        }
    }

    /// Construct from two points lying on edge.
    ///
    /// When looking from the first point to the second one, then the left side is free (outside) and the right side is occupied (inside).
    pub fn from_edge(a: Vec2, b: Vec2) -> Self {
        Self::from_normal(a, (b - a).perp().normalize())
    }

    pub fn distance(&self, point: Vec2) -> f32 {
        point.dot(self.normal) - self.offset
    }
}

impl Shape for HalfPlane {
    fn locate(&self, point: Vec2) -> Location {
        Location::from_distance(self.distance(point))
    }

    fn clump(&self) -> Clump {
        Clump {
            centroid: Vec2::INFINITY,
            area: f32::INFINITY,
        }
    }
}
