extern crate std;

use crate::{ArcPolygon, ArcVertex, Circle, Integrable, IntersectTo, Polygon};
use approx::assert_abs_diff_eq;
use core::f32::consts::PI;
use glam::Vec2;
use std::vec::Vec;

const TEST_EPS: f32 = 1e-6;

#[test]
fn moment_segments() {
    let disk = Circle {
        center: Vec2::new(2.345, 3.456),
        radius: 1.234,
    }
    .fill();
    let Circle { center, radius } = disk.edge();

    let arc_poly = ArcPolygon::new([
        ArcVertex {
            point: center + Vec2::new(0.0, -radius),
            sagitta: radius,
        },
        ArcVertex {
            point: center + Vec2::new(0.0, radius),
            sagitta: radius,
        },
    ]);

    assert_abs_diff_eq!(arc_poly.moment(), disk.moment(), epsilon = TEST_EPS);
}

#[test]
fn moment_arc_triangle() {
    let disk = Circle {
        center: Vec2::new(2.345, 3.456),
        radius: 1.234,
    }
    .fill();
    let Circle { center, radius } = disk.edge();

    let arc_poly = ArcPolygon::new([
        ArcVertex {
            point: center + radius * Vec2::from_angle(0.0),
            sagitta: 0.5 * radius,
        },
        ArcVertex {
            point: center + radius * Vec2::from_angle(2.0 * PI / 3.0),
            sagitta: 0.5 * radius,
        },
        ArcVertex {
            point: center + radius * Vec2::from_angle(4.0 * PI / 3.0),
            sagitta: 0.5 * radius,
        },
    ]);

    assert_abs_diff_eq!(arc_poly.moment(), disk.moment(), epsilon = TEST_EPS);
}

#[test]
fn intersect_polygon_circle_inside() {
    let disk = Circle {
        center: Vec2::new(2.345, 3.456),
        radius: 1.234,
    }
    .fill();
    let Circle { center, radius } = disk.edge();

    let poly = Polygon::new([
        center + 2.0 * radius * Vec2::from_angle(0.0),
        center + 2.0 * radius * Vec2::from_angle(2.0 * PI / 3.0),
        center + 2.0 * radius * Vec2::from_angle(4.0 * PI / 3.0),
    ]);

    let intersection: ArcPolygon<Vec<ArcVertex>> = poly.intersect_to(&disk).unwrap();

    assert_abs_diff_eq!(intersection.moment(), disk.moment(), epsilon = TEST_EPS);
}

#[test]
fn intersect_polygon_circle_sector() {
    let disk = Circle {
        center: Vec2::new(0.0, 0.0),
        radius: 1.0,
    }
    .fill();

    let poly = Polygon::new([
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(0.0, 2.0),
    ]);

    let intersection: ArcPolygon<Vec<ArcVertex>> = poly.intersect_to(&disk).unwrap();

    assert_abs_diff_eq!(intersection.area(), disk.area() / 4.0, epsilon = TEST_EPS);
}

#[test]
fn intersect_polygon_circle_stripe() {
    let disk = Circle {
        center: Vec2::new(0.0, 0.0),
        radius: 1.0,
    }
    .fill();

    let poly = Polygon::new([
        Vec2::new(-2.0, -0.5),
        Vec2::new(2.0, -0.5),
        Vec2::new(2.0, 0.5),
        Vec2::new(-2.0, 0.5),
    ]);

    let intersection: ArcPolygon<Vec<ArcVertex>> = poly.intersect_to(&disk).unwrap();
    assert_eq!(intersection.len(), 4);
    assert_abs_diff_eq!(
        intersection.area(),
        disk.area() / 3.0 + 3.0f32.sqrt() / 2.0,
        epsilon = TEST_EPS
    );
}
