use crate::{EPS, Intersect, Line, LineSegment};
use approx::assert_relative_eq;
use glam::Vec2;

// Helper macro for approximate equality
macro_rules! assert_vec2_eq {
    ($a:expr, $b:expr) => {
        assert_relative_eq!($a, $b, epsilon = EPS)
    };
}

#[test]
fn line_line_intersection() {
    // Basic intersection
    let l1 = Line(Vec2::new(0.0, 0.0), Vec2::new(2.0, 0.0));
    let l2 = Line(Vec2::new(1.0, -1.0), Vec2::new(1.0, 1.0));
    let result = l1.intersect(&l2);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 0.0));

    // Intersection at endpoint of line definition
    let l1 = Line(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let l2 = Line(Vec2::new(2.0, 0.0), Vec2::new(0.0, 2.0));
    let result = l1.intersect(&l2);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));
}

#[test]
fn line_line_parallel() {
    // Parallel lines (different intercepts)
    let l1 = Line(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
    let l2 = Line(Vec2::new(0.0, 1.0), Vec2::new(1.0, 2.0));
    let result = l1.intersect(&l2);
    assert!(result.is_none());

    // Vertical parallel lines
    let l1 = Line(Vec2::new(1.0, 0.0), Vec2::new(1.0, 10.0));
    let l2 = Line(Vec2::new(2.0, 0.0), Vec2::new(2.0, 10.0));
    let result = l1.intersect(&l2);
    assert!(result.is_none());
}

#[test]
fn line_line_coincident() {
    // Same line defined by different points
    let l1 = Line(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let l2 = Line(Vec2::new(-1.0, -1.0), Vec2::new(3.0, 3.0));
    let result = l1.intersect(&l2);
    assert!(result.is_some());
    // Should return any point on the line
    assert!(l1.is_near(result.unwrap()));

    // Coincident but opposite directions
    let l1 = Line(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let l2 = Line(Vec2::new(3.0, 3.0), Vec2::new(1.0, 1.0));
    let result = l1.intersect(&l2);
    assert!(result.is_some());
    // Should return any point on the line
    assert!(l1.is_near(result.unwrap()));
}

#[test]
fn line_line_degenerate() {
    // First line is degenerate (points too close)
    let l1 = Line(Vec2::new(1.0, 1.0), Vec2::new(1.0 + EPS / 2.0, 1.0));
    let l2 = Line(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let result = l1.intersect(&l2);
    // Degenerate line intersects if point lies on other line
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));

    // Second line is degenerate
    let l1 = Line(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let l2 = Line(Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0 + EPS / 2.0));
    let result = l1.intersect(&l2);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));

    // Both lines degenerate at same point
    let l1 = Line(Vec2::new(1.0, 1.0), Vec2::new(1.0 + EPS / 2.0, 1.0));
    let l2 = Line(Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0 + EPS / 2.0));
    let result = l1.intersect(&l2);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));

    // Both lines degenerate at different points
    let l1 = Line(Vec2::new(0.0, 0.0), Vec2::new(0.0 + EPS / 2.0, 0.0));
    let l2 = Line(Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0 + EPS / 2.0));
    let result = l1.intersect(&l2);
    assert!(result.is_none());
}

#[test]
fn line_segment_intersection() {
    // Segment fully intersects line
    let seg = LineSegment(Vec2::new(-1.0, 0.0), Vec2::new(3.0, 0.0));
    let line = Line(Vec2::new(1.0, -1.0), Vec2::new(1.0, 1.0));
    let result = seg.intersect(&line);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 0.0));

    // Segment endpoint lies on line
    let seg = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let line = Line(Vec2::new(2.0, 0.0), Vec2::new(0.0, 2.0));
    let result = seg.intersect(&line);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));

    // Segment just touches line at endpoint
    let seg = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
    let line = Line(Vec2::new(2.0, 0.0), Vec2::new(3.0, -1.0));
    let result = seg.intersect(&line);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));
}

#[test]
fn line_segment_no_intersection() {
    // Segment doesn't reach line
    let seg = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(0.5, 0.5));
    let line = Line(Vec2::new(2.0, 0.0), Vec2::new(0.0, 2.0));
    let result = seg.intersect(&line);
    assert!(result.is_none());

    // Segment beyond intersection point
    let seg = LineSegment(Vec2::new(2.0, 2.0), Vec2::new(3.0, 3.0));
    let line = Line(Vec2::new(0.0, 0.0), Vec2::new(10.0, 0.0));
    let result = seg.intersect(&line);
    assert!(result.is_none());
}

#[test]
fn line_segment_parallel() {
    // Segment parallel to line, not coincident
    let seg = LineSegment(Vec2::new(0.0, 1.0), Vec2::new(2.0, 1.0));
    let line = Line(Vec2::new(0.0, 0.0), Vec2::new(2.0, 0.0));
    let result = seg.intersect(&line);
    assert!(result.is_none());

    // Segment coincident with line
    let seg = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let line = Line(Vec2::new(-1.0, -1.0), Vec2::new(3.0, 3.0));
    let result = seg.intersect(&line);
    // Should return midpoint of segment
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));
}

#[test]
fn line_segment_degenerate() {
    // Degenerate segment on line
    let seg = LineSegment(Vec2::new(1.0, 1.0), Vec2::new(1.0 + EPS / 2.0, 1.0));
    let line = Line(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let result = seg.intersect(&line);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));

    // Degenerate segment not on line
    let seg = LineSegment(Vec2::new(0.0, 1.0), Vec2::new(0.0, 1.0 + EPS / 2.0));
    let line = Line(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let result = seg.intersect(&line);
    assert!(result.is_none());

    // Line is degenerate, segment intersects at degenerate point
    let seg = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let line = Line(Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0 + EPS / 2.0));
    let result = seg.intersect(&line);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));

    // Both degenerate, same point
    let seg = LineSegment(Vec2::new(1.0, 1.0), Vec2::new(1.0 + EPS / 2.0, 1.0));
    let line = Line(Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0 + EPS / 2.0));
    let result = seg.intersect(&line);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));
}

#[test]
fn line_segment_commutative() {
    // Test that Line.intersect(LineSegment) == LineSegment.intersect(Line)
    let line = Line(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let seg = LineSegment(Vec2::new(0.0, 2.0), Vec2::new(2.0, 0.0));

    let result1 = line.intersect(&seg);
    let result2 = seg.intersect(&line);

    assert!(result1.is_some());
    assert!(result2.is_some());
    assert_vec2_eq!(result1.unwrap(), result2.unwrap());
    assert_vec2_eq!(result1.unwrap(), Vec2::new(1.0, 1.0));
}

#[test]
fn segment_segment_intersection() {
    // Basic intersection
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let s2 = LineSegment(Vec2::new(0.0, 2.0), Vec2::new(2.0, 0.0));
    let result = s1.intersect(&s2);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));

    // Intersection at endpoint
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
    let s2 = LineSegment(Vec2::new(1.0, 1.0), Vec2::new(2.0, 0.0));
    let result = s1.intersect(&s2);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));

    // Just touching at endpoints
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
    let s2 = LineSegment(Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0));
    let result = s1.intersect(&s2);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 0.0));
}

#[test]
fn segment_segment_no_intersection() {
    // Segments don't intersect
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
    let s2 = LineSegment(Vec2::new(2.0, 0.0), Vec2::new(3.0, 0.0));
    let result = s1.intersect(&s2);
    assert!(result.is_none());

    // Lines intersect but segments don't
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
    let s2 = LineSegment(Vec2::new(2.0, 2.0), Vec2::new(3.0, 3.0));
    let result = s1.intersect(&s2);
    assert!(result.is_none());
}

#[test]
fn segment_segment_parallel() {
    // Parallel, not collinear
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(2.0, 0.0));
    let s2 = LineSegment(Vec2::new(0.0, 1.0), Vec2::new(2.0, 1.0));
    let result = s1.intersect(&s2);
    assert!(result.is_none());

    // Parallel, collinear, no overlap
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
    let s2 = LineSegment(Vec2::new(2.0, 0.0), Vec2::new(3.0, 0.0));
    let result = s1.intersect(&s2);
    assert!(result.is_none());
}

#[test]
fn segment_segment_collinear_overlap() {
    // Complete overlap
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(3.0, 0.0));
    let s2 = LineSegment(Vec2::new(1.0, 0.0), Vec2::new(2.0, 0.0));
    let result = s1.intersect(&s2);
    // Should return midpoint of overlap (1.5, 0.0)
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.5, 0.0));

    // Partial overlap
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(2.0, 0.0));
    let s2 = LineSegment(Vec2::new(1.0, 0.0), Vec2::new(3.0, 0.0));
    let result = s1.intersect(&s2);
    // Overlap from (1.0, 0.0) to (2.0, 0.0), midpoint at (1.5, 0.0)
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.5, 0.0));

    // Overlap at single point
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
    let s2 = LineSegment(Vec2::new(1.0, 0.0), Vec2::new(2.0, 0.0));
    let result = s1.intersect(&s2);
    // Should return the point (1.0, 0.0)
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 0.0));

    // Multiple overlapping segments (test t_min/t_max logic)
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(4.0, 0.0));
    let s2 = LineSegment(Vec2::new(1.0, 0.0), Vec2::new(3.0, 0.0));
    let result = s1.intersect(&s2);
    // Overlap from (1.0, 0.0) to (3.0, 0.0), midpoint at (2.0, 0.0)
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(2.0, 0.0));
}

#[test]
fn segment_segment_degenerate() {
    // First segment degenerate, lies on second segment
    let s1 = LineSegment(Vec2::new(1.0, 1.0), Vec2::new(1.0 + EPS / 2.0, 1.0));
    let s2 = LineSegment(Vec2::new(0.0, 2.0), Vec2::new(2.0, 0.0));
    let result = s1.intersect(&s2);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));

    // First segment degenerate, doesn't lie on second segment
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(0.0 + EPS / 2.0, 0.0));
    let s2 = LineSegment(Vec2::new(1.0, 1.0), Vec2::new(2.0, 2.0));
    let result = s1.intersect(&s2);
    assert!(result.is_none());

    // Second segment degenerate, lies on first segment
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    let s2 = LineSegment(Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0 + EPS / 2.0));
    let result = s1.intersect(&s2);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));

    // Both segments degenerate, same point
    let s1 = LineSegment(Vec2::new(1.0, 1.0), Vec2::new(1.0 + EPS / 2.0, 1.0));
    let s2 = LineSegment(Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0 + EPS / 2.0));
    let result = s1.intersect(&s2);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(1.0, 1.0));

    // Both segments degenerate, different points
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(0.0 + EPS / 2.0, 0.0));
    let s2 = LineSegment(Vec2::new(1.0, 1.0), Vec2::new(1.0, 1.0 + EPS / 2.0));
    let result = s1.intersect(&s2);
    assert!(result.is_none());
}

#[test]
fn segment_segment_horizontal_vertical() {
    // Horizontal and vertical segments
    let horiz = LineSegment(Vec2::new(0.0, 2.0), Vec2::new(5.0, 2.0));
    let vert = LineSegment(Vec2::new(3.0, 0.0), Vec2::new(3.0, 5.0));
    let result = horiz.intersect(&vert);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(3.0, 2.0));

    // Vertical segment endpoints exactly on horizontal
    let horiz = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(5.0, 0.0));
    let vert = LineSegment(Vec2::new(2.0, -1.0), Vec2::new(2.0, 1.0));
    let result = horiz.intersect(&vert);
    assert!(result.is_some());
    assert_vec2_eq!(result.unwrap(), Vec2::new(2.0, 0.0));
}

#[test]
fn segment_segment_numerical_stability() {
    // Test with very small/large coordinates
    let s1 = LineSegment(Vec2::new(1e-9, 1e-9), Vec2::new(1e+9, 1e+9));
    let s2 = LineSegment(Vec2::new(1e+9, 1e-9), Vec2::new(1e-9, 1e+9));
    let result = s1.intersect(&s2);
    assert!(result.is_some());
    let expected = Vec2::new(5e8, 5e8);
    assert_relative_eq!(result.unwrap().x, expected.x, epsilon = 1e-3);
    assert_relative_eq!(result.unwrap().y, expected.y, epsilon = 1e-3);

    // Near parallel segments (small angle)
    let s1 = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(1000.0, 0.0));
    let s2 = LineSegment(Vec2::new(0.0, 0.001), Vec2::new(1000.0, 0.0));
    let result = s1.intersect(&s2);
    // Should intersect near (500.0, 0.0) but with slight y offset
    assert!(result.is_some());
}

#[test]
fn contains() {
    let seg = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(2.0, 0.0));

    // Point at start
    assert!(seg.is_near(Vec2::new(0.0, 0.0)));

    // Point at end
    assert!(seg.is_near(Vec2::new(2.0, 0.0)));

    // Point in middle
    assert!(seg.is_near(Vec2::new(1.0, 0.0)));

    // Point slightly beyond start (within epsilon)
    assert!(seg.is_near(Vec2::new(-EPS / 2.0, 0.0)));

    // Point slightly beyond end (within epsilon)
    assert!(seg.is_near(Vec2::new(2.0 + EPS / 2.0, 0.0)));

    // Point not on segment
    assert!(!seg.is_near(Vec2::new(3.0, 0.0)));

    // Point not collinear
    assert!(!seg.is_near(Vec2::new(1.0, 1.0)));

    // Diagonal segment
    let seg = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0));
    assert!(seg.is_near(Vec2::new(1.0, 1.0)));
    assert!(!seg.is_near(Vec2::new(1.0, 1.1)));
}

#[test]
fn contains_degenerate() {
    // Degenerate segment (zero length)
    let seg = LineSegment(Vec2::new(1.0, 1.0), Vec2::new(1.0 + EPS / 2.0, 1.0));

    // Contains its own points
    assert!(seg.is_near(Vec2::new(1.0, 1.0)));
    assert!(seg.is_near(Vec2::new(1.0 + EPS / 2.0, 1.0)));

    // Doesn't contain other points
    assert!(!seg.is_near(Vec2::new(1.0, 1.1)));
}

#[test]
fn is_degenerate() {
    // Normal line/segment
    let line = Line(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
    assert!(!line.is_degenerate());

    // Degenerate line
    let line = Line(Vec2::new(0.0, 0.0), Vec2::new(EPS / 2.0, 0.0));
    assert!(line.is_degenerate());

    // Normal segment
    let seg = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0));
    assert!(!seg.is_degenerate());

    // Degenerate segment
    let seg = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(0.0, EPS / 2.0));
    assert!(seg.is_degenerate());
}

#[test]
fn intersection_symmetry() {
    // Test that intersection is symmetric: a.intersect(b) == b.intersect(a)
    let segments = [
        LineSegment(Vec2::new(0.0, 0.0), Vec2::new(2.0, 2.0)),
        LineSegment(Vec2::new(0.0, 2.0), Vec2::new(2.0, 0.0)),
        LineSegment(Vec2::new(1.0, 0.0), Vec2::new(1.0, 3.0)),
        LineSegment(Vec2::new(-1.0, -1.0), Vec2::new(3.0, 3.0)),
    ];

    for i in 0..segments.len() {
        for j in 0..segments.len() {
            let a = &segments[i];
            let b = &segments[j];
            let result1 = a.intersect(b);
            let result2 = b.intersect(a);

            match (result1, result2) {
                (Some(p1), Some(p2)) => assert_vec2_eq!(p1, p2),
                (None, None) => (),
                _ => panic!("Intersection not symmetric between {} and {}", i, j),
            }
        }
    }
}
