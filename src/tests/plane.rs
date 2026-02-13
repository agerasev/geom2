use crate::{Closed, HalfPlane, Line};
use approx::assert_abs_diff_eq;
use core::f32::consts::PI;
use glam::Vec2;

const TEST_EPS: f32 = 1e-6;

#[test]
fn is_inside() {
    let plane = HalfPlane::from_edge(Line(Vec2::new(0.0, 1.0), Vec2::new(1.0, 0.0)));

    // Points on the right side should be outside
    assert!(!plane.contains(Vec2::new(0.0, 0.0)));
    // Points on the left side should be inside
    assert!(plane.contains(Vec2::new(1.0, 1.0)));
}

#[test]
fn from_normal() {
    let point = Vec2::new(2.0, 3.0);
    let normal = Vec2::new(1.0, 0.0).normalize();

    let plane = HalfPlane::from_normal(point, normal);

    assert_eq!(plane.distance(point), 0.0);

    let outside_point = Vec2::new(3.0, 3.0);
    assert!(!plane.contains(outside_point));
    assert!(plane.distance(outside_point) > 0.0);

    let inside_point = Vec2::new(1.0, 3.0);
    assert!(plane.contains(inside_point));
    assert!(plane.distance(inside_point) < 0.0);
}

#[test]
fn from_edge() {
    {
        // Horizontal edge from (0,0) to (1,0)
        let plane = HalfPlane::from_edge(Line(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0)));

        // When looking from (0,0) to (1,0), left side is +Y direction
        assert!(plane.contains(Vec2::new(0.5, 1.0))); // Above edge - inside
        assert!(!plane.contains(Vec2::new(0.5, -1.0))); // Below edge - outside
    }

    {
        // Vertical edge from (0,0) to (0,1)
        let plane = HalfPlane::from_edge(Line(Vec2::new(0.0, 0.0), Vec2::new(0.0, 1.0)));

        // When looking from (0,0) to (0,1), left side is -X direction
        assert!(plane.contains(Vec2::new(-1.0, 0.5))); // Left of edge - inside
        assert!(!plane.contains(Vec2::new(1.0, 0.5))); // Right of edge - outside
    }
}

#[test]
fn distance() {
    {
        let plane = HalfPlane {
            normal: Vec2::new(0.0, 1.0),
            offset: 2.0,
        };

        assert_abs_diff_eq!(
            plane.distance(Vec2::new(10.0, 2.0)),
            0.0,
            epsilon = TEST_EPS
        );
        assert_abs_diff_eq!(
            plane.distance(Vec2::new(10.0, 3.0)),
            1.0,
            epsilon = TEST_EPS
        );
        assert_abs_diff_eq!(
            plane.distance(Vec2::new(10.0, 1.0)),
            -1.0,
            epsilon = TEST_EPS
        );
    }

    {
        let point_on_boundary = Vec2::new(2.0, 2.0);
        let plane = HalfPlane::from_normal(point_on_boundary, Vec2::from_angle(PI / 4.0));

        // Point on boundary should have zero distance
        assert_abs_diff_eq!(plane.distance(point_on_boundary), 0.0, epsilon = TEST_EPS);

        // Move along normal direction (outside)
        let outside_point = point_on_boundary + plane.normal * 2.0;
        assert!(!plane.contains(outside_point));
        assert_abs_diff_eq!(plane.distance(outside_point), 2.0, epsilon = TEST_EPS);

        // Move opposite to normal direction (inside)
        let inside_point = point_on_boundary - plane.normal * 2.0;
        assert!(plane.contains(inside_point));
        assert_abs_diff_eq!(plane.distance(inside_point), -2.0, epsilon = TEST_EPS);
    }
}

#[test]
fn boundary_point() {
    let plane = HalfPlane {
        normal: Vec2::new(0.8, 0.6).normalize(),
        offset: -5.0,
    };

    let boundary_point = plane.boundary_point();
    assert_abs_diff_eq!(plane.distance(boundary_point), 0.0, epsilon = TEST_EPS);
}

#[test]
fn edge() {
    let plane = HalfPlane {
        normal: Vec2::new(0.0, 1.0),
        offset: -3.0,
    };

    let line = plane.edge();

    // Check that start point is on boundary
    assert_abs_diff_eq!(plane.distance(line.0), 0.0, epsilon = TEST_EPS);

    // Check that direction is perpendicular to normal
    let line_dir = line.1 - line.0;
    assert_abs_diff_eq!(line_dir.dot(plane.normal), 0.0, epsilon = TEST_EPS);

    // Check that second point is also on boundary
    assert_abs_diff_eq!(plane.distance(line.1), 0.0, epsilon = TEST_EPS);
}
