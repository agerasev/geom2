use crate::{Clump, Line, Shape};
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

    /// Get some point on the boundary line
    fn boundary_point(&self) -> Vec2 {
        self.normal * (-self.offset)
    }

    pub fn edge(&self) -> Line {
        let p = self.boundary_point();
        Line(p, p + self.normal.perp())
    }
}

impl Shape for HalfPlane {
    fn is_inside(&self, point: Vec2) -> bool {
        self.distance(point) <= 0.0
    }

    fn clump(&self) -> Clump {
        Clump {
            centroid: Vec2::INFINITY,
            area: f32::INFINITY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_inside() {
        let plane = HalfPlane::from_edge(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));

        // Points below the edge (y < 0) should be inside
        assert!(plane.is_inside(Vec2::new(0.0, -1.0)));
        // Points above the edge (y > 0) should be outside
        assert!(!plane.is_inside(Vec2::new(0.0, 1.0)));
        // Points on the edge should be inside
        assert!(plane.is_inside(Vec2::new(0.5, 0.0)));
    }
}
