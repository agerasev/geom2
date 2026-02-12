use crate::{ArcVertex, Integrate, Polygon};
use approx::assert_abs_diff_eq;
use core::f32::consts::PI;
use glam::Vec2;

const TEST_EPS: f32 = 1e-6;

#[test]
fn moment_segments() {
    let offset: Vec2 = Vec2::new(2.345, 3.456);
    let radius: f32 = 1.234;

    let arc_poly = Polygon::new([
        ArcVertex {
            point: offset + Vec2::new(0.0, -radius),
            sagitta: radius,
        },
        ArcVertex {
            point: offset + Vec2::new(0.0, radius),
            sagitta: radius,
        },
    ]);

    assert_abs_diff_eq!(arc_poly.area(), PI * radius.powi(2), epsilon = TEST_EPS);
    assert_abs_diff_eq!(arc_poly.centroid(), offset, epsilon = TEST_EPS);
}

#[test]
fn moment_arc_triangle() {
    let offset: Vec2 = Vec2::new(2.345, 3.456);
    let radius: f32 = 1.234;

    let arc_poly = Polygon::new([
        ArcVertex {
            point: offset + radius * Vec2::from_angle(0.0),
            sagitta: 0.5 * radius,
        },
        ArcVertex {
            point: offset + radius * Vec2::from_angle(2.0 * PI / 3.0),
            sagitta: 0.5 * radius,
        },
        ArcVertex {
            point: offset + radius * Vec2::from_angle(4.0 * PI / 3.0),
            sagitta: 0.5 * radius,
        },
    ]);

    assert_abs_diff_eq!(arc_poly.area(), PI * radius.powi(2), epsilon = TEST_EPS);
    assert_abs_diff_eq!(arc_poly.centroid(), offset, epsilon = TEST_EPS);
}
