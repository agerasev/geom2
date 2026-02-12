use crate::{
    AsIterator, Bounded, EPS, HalfPlane, Integrate, IntersectTo, LineSegment, Moment, Polygon,
};
use glam::Vec2;

impl<V: AsIterator<Item = Vec2> + ?Sized> Polygon<V> {
    pub fn is_convex(&self) -> bool {
        let mut sign = 0.0;
        for [a, b, c] in self.vertices_window() {
            let cross = (b - a).perp_dot(c - b);

            if sign == 0.0 {
                sign = cross;
            } else if sign * cross < 0.0 {
                return false;
            }
        }
        true
    }
}

impl<V: AsIterator<Item = Vec2> + ?Sized> Bounded for Polygon<V> {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        let mut winding_number = 0;

        for LineSegment(v0, v1) in self.edges() {
            // Test if edge crosses the horizontal line at point.y
            if v0.y <= point.y {
                if v1.y > point.y {
                    // Upward crossing - check if point is left of edge
                    if (v1 - v0).perp_dot(point - v0) > 0.0 {
                        winding_number += 1;
                    }
                }
            } else if v1.y <= point.y {
                // Downward crossing - check if point is right of edge
                if (v1 - v0).perp_dot(point - v0) < 0.0 {
                    winding_number -= 1;
                }
            }
        }

        winding_number
    }
}

impl<V: AsIterator<Item = Vec2> + ?Sized> Integrate for Polygon<V> {
    fn moment(&self) -> Moment {
        // Shoelace formula
        let mut area = 0.0;
        let mut centroid = Vec2::ZERO;
        for LineSegment(a, b) in self.edges() {
            let cross = a.perp_dot(b);
            area += cross;
            centroid += (a + b) * cross;
        }
        area = area.abs() * 0.5;
        if area < EPS {
            centroid = Vec2::ZERO;
        } else {
            centroid /= 6.0 * area;
        }
        Moment { area, centroid }
    }
}

impl<V: AsIterator<Item = Vec2> + ?Sized, W: AsIterator<Item = Vec2> + FromIterator<Vec2>>
    IntersectTo<HalfPlane, Polygon<W>> for Polygon<V>
{
    fn intersect_to(&self, plane: &HalfPlane) -> Option<Polygon<W>> {
        let mut prev = match self.vertices().last() {
            Some(p) => *p,
            None => return None,
        };
        let mut prev_inside = plane.contains(prev);
        let clip_iter = self
            .vertices()
            .copied()
            .flat_map(|v| {
                let inside = plane.contains(v);
                let ret = match (prev_inside, inside) {
                    (true, true) => [None, Some(v)],
                    (true, false) => [
                        None,
                        Some(plane.edge().intersect_to(&LineSegment(prev, v)).unwrap()),
                    ],
                    (false, true) => [
                        Some(plane.edge().intersect_to(&LineSegment(prev, v)).unwrap()),
                        Some(v),
                    ],
                    (false, false) => [None, None],
                };
                prev_inside = inside;
                prev = v;
                ret
            })
            .flatten();
        let result = Polygon::<W>::from_iter(clip_iter);
        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }
}

impl<V: AsIterator<Item = Vec2> + ?Sized, W: AsIterator<Item = Vec2> + FromIterator<Vec2>>
    IntersectTo<Polygon<V>, Polygon<W>> for HalfPlane
{
    fn intersect_to(&self, other: &Polygon<V>) -> Option<Polygon<W>> {
        other.intersect_to(self)
    }
}

impl<
    U: AsIterator<Item = Vec2> + ?Sized,
    V: AsIterator<Item = Vec2> + ?Sized,
    W: AsIterator<Item = Vec2> + FromIterator<Vec2>,
> IntersectTo<Polygon<U>, Polygon<W>> for Polygon<V>
{
    fn intersect_to(&self, other: &Polygon<U>) -> Option<Polygon<W>> {
        let mut result = Polygon::from_iter(self.vertices().copied());

        // Sutherland-Hodgman polygon clipping algorithm
        for LineSegment(a, b) in other.edges() {
            let plane = HalfPlane::from_edge(a, b);
            result = result.intersect_to(&plane)?;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
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
                Vec2::new(1.0, 1.0),
                Vec2::new(2.0, 1.0),
                Vec2::new(2.0, 2.0),
                Vec2::new(1.0, 2.0),
            ])
        )
    }
}
