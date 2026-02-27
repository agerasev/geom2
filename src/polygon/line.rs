use crate::{
    AsIterator, Closed, EPS, GenericPolygon, HalfPlane, Integrable, IntersectTo, Line, LineSegment,
    Moment,
};
use genawaiter::{stack::let_gen, yield_};
use glam::Vec2;

pub type Polygon<V> = GenericPolygon<V, Vec2>;

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

impl<V: AsIterator<Item = Vec2> + ?Sized> Closed for Polygon<V> {
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

impl<V: AsIterator<Item = Vec2> + ?Sized> Integrable for Polygon<V> {
    fn moment(&self) -> Moment {
        // Shoelace formula
        let mut area = 0.0;
        let mut centroid = Vec2::ZERO;
        for LineSegment(a, b) in self.edges() {
            let cross = a.perp_dot(b);
            area += cross;
            centroid += (a + b) * cross;
        }
        area *= 0.5;
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
        // Clip vertices
        let_gen!(gen_, {
            let mut iter = self.vertices();
            let mut prev = match iter.next() {
                Some(x) => x,
                None => return,
            };
            let mut prev_dist = plane.distance(prev);
            for v in iter.chain([prev]) {
                let dist = plane.distance(v);
                if prev_dist < 0.0 {
                    // prev inside
                    if dist < 0.0 {
                        // v inside
                        yield_!(prev);
                    } else {
                        // v outside
                        yield_!(prev);

                        let sum_dist = dist - prev_dist;
                        yield_!(if sum_dist < EPS {
                            0.5 * (prev + v)
                        } else {
                            (prev * dist - v * prev_dist) / sum_dist
                        });
                    }
                } else {
                    // prev outside
                    if dist < 0.0 {
                        // v inside
                        let sum_dist = prev_dist - dist;
                        yield_!(if sum_dist < EPS {
                            0.5 * (prev + v)
                        } else {
                            (v * prev_dist - prev * dist) / sum_dist
                        });
                    } else {
                        // v outside
                        // do nothing
                    }
                }
                prev_dist = dist;
                prev = v;
            }
        });
        let mut iter = gen_.into_iter();

        if let Some(mut prev) = iter.next() {
            // Deduplicate vertices
            let iter = iter.chain([prev]).filter_map(|v| {
                let ret = if (prev - v).abs().max_element() > EPS {
                    Some(prev)
                } else {
                    None
                };
                prev = v;
                ret
            });
            Some(Polygon::<W>::from_iter(iter))
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
        let mut result = Polygon::from_iter(self.vertices());

        // Sutherland-Hodgman polygon clipping algorithm
        for LineSegment(a, b) in other.edges() {
            let plane = HalfPlane::from_edge(Line(a, b));
            result = result.intersect_to(&plane)?;
        }

        Some(result)
    }
}
