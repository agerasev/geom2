use crate::{Bound, Line};
use glam::Vec2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HalfPlane {
    /// Normal of the half-plane edge, pointing from free space to occuped space.
    pub normal: Vec2,
    /// Minimal distance from the origin to the half-plane edge.
    ///
    /// If the origin is outside of half-plane then it is positive, when origin is inside  it is negative.
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
    /// When looking from the first point to the second one, then the left side is occupied (inside) and the right side is free (outside).
    pub fn from_edge(a: Vec2, b: Vec2) -> Self {
        Self::from_normal(a, (b - a).perp().normalize())
    }

    /// Minimal distance to the edge from the `point`.
    /// It is positive if `point` is inside of the half-plane, and negative if outside.
    pub fn distance(&self, point: Vec2) -> f32 {
        point.dot(self.normal) - self.offset
    }

    /// Get some point on the edge.
    fn boundary_point(&self) -> Vec2 {
        self.normal * self.offset
    }

    pub fn edge(&self) -> Line {
        let p = self.boundary_point();
        Line(p, p - self.normal.perp())
    }
}

impl Bound for HalfPlane {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        self.distance(point).signum() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts::PI;
    use glam::Vec2;

    #[test]
    fn is_inside() {
        let plane = HalfPlane::from_edge(Vec2::new(0.0, 1.0), Vec2::new(1.0, 0.0));

        // Points on the right side should be outside
        assert!(!plane.contains(Vec2::new(0.0, 0.0)));
        // Points on the left side should be inside
        assert!(plane.contains(Vec2::new(1.0, 1.0)));
        // Points on the edge should be inside
        assert!(plane.contains(Vec2::new(0.5, 0.5)));
    }

    #[test]
    fn from_normal_construction() {
        let point = Vec2::new(2.0, 3.0);
        let normal = Vec2::new(1.0, 0.0).normalize(); // Unit vector pointing right

        let plane = HalfPlane::from_normal(point, normal);

        // Point used for construction should be exactly on the boundary
        assert_eq!(plane.distance(point), 0.0);

        // Points to the right of normal should be positive distance (outside)
        assert!(plane.distance(Vec2::new(3.0, 3.0)) > 0.0);
        // Points to the left of normal should be negative distance (inside)
        assert!(plane.distance(Vec2::new(1.0, 3.0)) < 0.0);
    }

    #[test]
    fn from_edge_direction_consistency() {
        // Test that the "left side is occupied" rule is consistent

        // Horizontal edge from (0,0) to (1,0)
        let plane = HalfPlane::from_edge(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));

        // When looking from (0,0) to (1,0), left side is +Y direction
        assert!(plane.contains(Vec2::new(0.5, 1.0))); // Above edge - inside
        assert!(!plane.contains(Vec2::new(0.5, -1.0))); // Below edge - outside

        // Vertical edge from (0,0) to (0,1)
        let plane = HalfPlane::from_edge(Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0));

        // When looking from (0,0) to (0,1), left side is -X direction
        assert!(plane.contains(Vec2::new(-1.0, 0.5))); // Left of edge - inside
        assert!(!plane.contains(Vec2::new(1.0, 0.5))); // Right of edge - outside
    }

    #[test]
    fn distance_calculation() {
        // Simple horizontal half-plane at y = 2, normal pointing down
        let plane = HalfPlane {
            normal: Vec2::new(0.0, 1.0), // Pointing up
            offset: -2.0,                // Boundary at y = 2
        };

        // distance = point.y * 1.0 + (-2.0) = point.y - 2.0
        assert_eq!(plane.distance(Vec2::new(10.0, 2.0)), 0.0); // On boundary
        assert_eq!(plane.distance(Vec2::new(10.0, 3.0)), 1.0); // 1 unit outside
        assert_eq!(plane.distance(Vec2::new(10.0, 1.0)), -1.0); // 1 unit inside

        // Test with rotated normal
        let angle = PI / 4.0; // 45 degrees
        let normal = Vec2::new(angle.cos(), angle.sin());
        let point_on_boundary = Vec2::new(2.0, 2.0);

        let plane = HalfPlane::from_normal(point_on_boundary, normal);

        // Point on boundary should have zero distance
        assert!((plane.distance(point_on_boundary).abs() < 1e-6));

        // Move along normal direction (outside)
        let outside_point = point_on_boundary + normal * 2.0;
        assert!((plane.distance(outside_point) - 2.0).abs() < 1e-6);

        // Move opposite to normal direction (inside)
        let inside_point = point_on_boundary - normal * 2.0;
        assert!((plane.distance(inside_point) + 2.0).abs() < 1e-6);
    }

    #[test]
    fn boundary_point() {
        let plane = HalfPlane {
            normal: Vec2::new(0.8, 0.6).normalize(),
            offset: -5.0,
        };

        let boundary_point = plane.boundary_point();

        // The boundary point should satisfy: normal·boundary_point + offset = 0
        let distance_at_boundary = plane.distance(boundary_point);
        assert!((distance_at_boundary.abs() < 1e-6));

        // Check that boundary_point = normal * (-offset)
        let expected = plane.normal * (-plane.offset);
        assert!((boundary_point - expected).length() < 1e-6);
    }

    #[test]
    fn edge_creation() {
        let plane = HalfPlane {
            normal: Vec2::new(0.0, 1.0), // Pointing up
            offset: -3.0,                // Boundary at y = 3
        };

        let line = plane.edge();

        // Line should be horizontal at y = 3
        // boundary_point = (0, 3), perp = (-1, 0)
        // So line goes from (0, 3) to (-1, 3)

        // Check that start point is on boundary
        assert!((plane.distance(line.0).abs() < 1e-6));

        // Check that direction is perpendicular to normal
        let line_dir = line.1 - line.0;
        assert!((line_dir.dot(plane.normal).abs() < 1e-6));

        // Check that second point is also on boundary
        assert!((plane.distance(line.1).abs() < 1e-6));
    }

    #[test]
    fn special_cases() {
        // Half-plane through origin
        let plane = HalfPlane {
            normal: Vec2::new(1.0, 0.0),
            offset: 0.0,
        };
        assert_eq!(plane.distance(Vec2::ZERO), 0.0);
        assert_eq!(plane.boundary_point(), Vec2::ZERO);

        // Half-plane with negative normal
        let plane = HalfPlane {
            normal: Vec2::new(-1.0, 0.0),
            offset: 3.0,
        };
        // boundary_point = (-1, 0) * (-3) = (3, 0)
        assert!((plane.boundary_point() - Vec2::new(3.0, 0.0)).length() < 1e-6);

        // Test with non-normalized input to from_normal (should still work if caller ensures normalization)
        let point = Vec2::new(1.0, 2.0);
        let non_unit_normal = Vec2::new(2.0, 0.0); // Length 2
        let unit_normal = non_unit_normal.normalize();
        let plane1 = HalfPlane::from_normal(point, unit_normal);

        // Manually create expected half-plane
        let expected_offset = -point.dot(unit_normal);
        assert_eq!(plane1.offset, expected_offset);
    }

    #[test]
    fn edge_cases_numerical_stability() {
        // Very small normal
        let tiny = 1e-10;
        let plane = HalfPlane {
            normal: Vec2::new(tiny, 0.0),
            offset: -1.0,
        };

        // Boundary point calculation shouldn't overflow
        let bp = plane.boundary_point();
        assert!(bp.x.is_finite());
        assert!(bp.y.is_finite());

        // Very large offset
        let plane = HalfPlane {
            normal: Vec2::new(1.0, 0.0),
            offset: -1e10,
        };

        let bp = plane.boundary_point();
        assert_eq!(bp.x, 1e10);
        assert_eq!(bp.y, 0.0);

        // Points far from boundary
        let plane = HalfPlane::from_edge(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
        let far_point = Vec2::new(0.5, 1e6);
        assert!(plane.contains(far_point));

        let far_outside = Vec2::new(0.5, -1e6);
        assert!(!plane.contains(far_outside));
    }
}
