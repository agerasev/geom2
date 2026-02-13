extern crate std;

use crate::{Closed, HalfPlane, Integrable, IntersectTo, Moment, Polygon};
use glam::Vec2;
use std::vec::Vec;

#[test]
fn square_clump() {
    let square = Polygon::new([
        Vec2::new(0.0, 0.0),
        Vec2::new(3.0, 0.0),
        Vec2::new(3.0, 2.0),
        Vec2::new(0.0, 2.0),
    ]);
    assert_eq!(
        square.moment(),
        Moment {
            area: 6.0,
            centroid: Vec2::new(1.5, 1.0)
        }
    )
}

#[test]
fn is_inside() {
    // Triangle
    let triangle = Polygon::new([
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(1.0, 2.0),
    ]);

    // Points inside triangle
    assert!(triangle.contains(Vec2::new(1.0, 0.5)));
    assert!(triangle.contains(Vec2::new(0.5, 0.5)));
    assert!(triangle.contains(Vec2::new(1.5, 0.5)));

    // Points outside triangle
    assert!(!triangle.contains(Vec2::new(3.0, 3.0)));
    assert!(!triangle.contains(Vec2::new(-1.0, -1.0)));

    // Test with complex concave polygon
    let concave = Polygon::new([
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(1.0, 2.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ]);

    // Points in the concave region should be outside
    assert!(!concave.contains(Vec2::new(0.5, 1.5)));
    // Points in the main region should be inside
    assert!(concave.contains(Vec2::new(1.5, 0.5)));
}

#[test]
fn is_convex() {
    // Convex polygon (triangle)
    let triangle = Polygon::new([
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(1.0, 2.0),
    ]);
    assert!(triangle.is_convex());

    // Convex polygon (square)
    let square = Polygon::new([
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(0.0, 2.0),
    ]);
    assert!(square.is_convex());

    // Concave polygon
    let concave = Polygon::new([
        Vec2::new(0.0, 0.0),
        Vec2::new(3.0, 0.0),
        Vec2::new(3.0, 2.0),
        Vec2::new(1.0, 2.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ]);
    assert!(!concave.is_convex());

    // Degenerate cases
    let empty: Polygon<[Vec2; 0]> = Polygon::new([]);
    assert!(empty.is_convex());

    let point = Polygon::new([Vec2::ZERO]);
    assert!(point.is_convex());

    let line = Polygon::new([Vec2::ZERO, Vec2::ONE]);
    assert!(line.is_convex());
}

#[test]
fn intersect_plane() {
    let square = Polygon::new([
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(0.0, 2.0),
    ]);

    // Clip with a vertical plane at x = 1
    let plane = HalfPlane::from_normal(Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.0));
    let clipped: Polygon<Vec<Vec2>> = square.intersect_to(&plane).unwrap();

    // Should get a rectangle from x=0 to x=1
    assert_eq!(
        clipped,
        Polygon::new([
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(0.0, 2.0),
        ])
    );
}

#[test]
fn intersect_polygon() {
    let square1 = Polygon::new([
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(0.0, 2.0),
    ]);

    let square2 = Polygon::new([
        Vec2::new(1.0, 1.0),
        Vec2::new(3.0, 1.0),
        Vec2::new(3.0, 3.0),
        Vec2::new(1.0, 3.0),
    ]);

    let intersection: Polygon<Vec<Vec2>> = square1.intersect_to(&square2).unwrap();
    assert_eq!(
        intersection,
        Polygon::new([
            Vec2::new(2.0, 1.0),
            Vec2::new(2.0, 2.0),
            Vec2::new(1.0, 2.0),
            Vec2::new(1.0, 1.0),
        ])
    )
}
