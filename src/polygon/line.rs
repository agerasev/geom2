use crate::{
    Closed, CopyIterator, EPS, GenericPolygon, HalfPlane, Integrable, IntersectTo, Line,
    LineSegment, Meta, Moment, meta::Unmeta, polygon::FramedPolygon,
};
use genawaiter::{stack::let_gen, yield_};
use glam::Vec2;

pub type Polygon<V> = GenericPolygon<V, Vec2>;

impl<V: CopyIterator<Item = Vec2> + ?Sized> FramedPolygon for Polygon<V> {
    fn frame(&self) -> Polygon<impl CopyIterator<Item = Vec2> + '_> {
        Polygon::new(self.vertices.to_ref())
    }
}

impl<V: CopyIterator<Item = Vec2> + ?Sized> Polygon<V> {
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

impl<V: CopyIterator<Item = Vec2> + ?Sized> Closed for Polygon<V> {
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

impl<V: CopyIterator<Item = Vec2> + ?Sized> Integrable for Polygon<V> {
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

impl<V: CopyIterator<Item = Vec2> + ?Sized, W: CopyIterator<Item = Vec2> + FromIterator<Vec2>>
    IntersectTo<HalfPlane, Polygon<W>> for Polygon<V>
{
    fn intersect_to(&self, plane: &HalfPlane) -> Option<Polygon<W>> {
        let unmeta: GenericPolygon<Unmeta<W>, Meta<Vec2, ()>> =
            Meta::new(Polygon::new(self.vertices.to_ref()), ())
                .intersect_to(&Meta::new(*plane, ()))?;
        Some(Polygon::new(unmeta.vertices.0))
    }
}

impl<V: CopyIterator<Item = Vec2> + ?Sized, W: CopyIterator<Item = Vec2> + FromIterator<Vec2>>
    IntersectTo<Polygon<V>, Polygon<W>> for HalfPlane
{
    fn intersect_to(&self, other: &Polygon<V>) -> Option<Polygon<W>> {
        other.intersect_to(self)
    }
}

impl<
    M: Copy,
    V: CopyIterator<Item = Meta<Vec2, M>> + ?Sized,
    W: CopyIterator<Item = Meta<Vec2, M>> + FromIterator<Meta<Vec2, M>>,
> IntersectTo<Meta<HalfPlane, M>, GenericPolygon<W, Meta<Vec2, M>>>
    for GenericPolygon<V, Meta<Vec2, M>>
{
    fn intersect_to(&self, plane: &Meta<HalfPlane, M>) -> Option<GenericPolygon<W, Meta<Vec2, M>>> {
        // Clip vertices
        let_gen!(gen_, {
            let mut iter = self.vertices();
            let mut prev = match iter.next() {
                Some(x) => x,
                None => return,
            };
            let mut prev_dist = plane.distance(*prev);
            for curr in iter.chain([prev]) {
                let dist = plane.distance(*curr);
                if prev_dist < 0.0 {
                    // prev inside
                    if dist < 0.0 {
                        // curr inside
                        yield_!(prev);
                    } else {
                        // curr outside
                        yield_!(prev);

                        let sum_dist = dist - prev_dist;
                        yield_!(Meta::new(
                            if sum_dist < EPS {
                                0.5 * (*prev + *curr)
                            } else {
                                (*prev * dist - *curr * prev_dist) / sum_dist
                            },
                            plane.meta
                        ));
                    }
                } else {
                    // prev outside
                    if dist < 0.0 {
                        // curr inside
                        let sum_dist = prev_dist - dist;
                        yield_!(Meta::new(
                            if sum_dist < EPS {
                                0.5 * (*prev + *curr)
                            } else {
                                (*curr * prev_dist - *prev * dist) / sum_dist
                            },
                            prev.meta
                        ));
                    } else {
                        // curr outside
                        // do nothing
                    }
                }
                prev_dist = dist;
                prev = curr;
            }
        });
        let mut iter = gen_.into_iter();

        if let Some(mut prev) = iter.next() {
            // Deduplicate vertices
            let iter = iter.chain([prev]).filter_map(|curr| {
                let ret = if (*prev - *curr).abs().max_element() > EPS {
                    Some(curr)
                } else {
                    None
                };
                prev = curr;
                ret
            });
            Some(GenericPolygon::<W, Meta<Vec2, M>>::from_iter(iter))
        } else {
            None
        }
    }
}

impl<
    M: Copy,
    V: CopyIterator<Item = Meta<Vec2, M>> + ?Sized,
    W: CopyIterator<Item = Meta<Vec2, M>> + FromIterator<Meta<Vec2, M>>,
> IntersectTo<GenericPolygon<V, Meta<Vec2, M>>, GenericPolygon<W, Meta<Vec2, M>>>
    for Meta<HalfPlane, M>
{
    fn intersect_to(
        &self,
        other: &GenericPolygon<V, Meta<Vec2, M>>,
    ) -> Option<GenericPolygon<W, Meta<Vec2, M>>> {
        other.intersect_to(self)
    }
}

impl<
    M: Copy,
    V: CopyIterator<Item = Vec2> + ?Sized,
    W: CopyIterator<Item = Meta<Vec2, M>> + FromIterator<Meta<Vec2, M>>,
> IntersectTo<Meta<HalfPlane, M>, GenericPolygon<W, Meta<Vec2, M>>> for Meta<Polygon<V>, M>
{
    fn intersect_to(&self, plane: &Meta<HalfPlane, M>) -> Option<GenericPolygon<W, Meta<Vec2, M>>> {
        GenericPolygon::new(self.vertices.map(|v| Meta::new(v, self.meta))).intersect_to(plane)
    }
}

impl<
    M: Copy,
    V: CopyIterator<Item = Vec2> + ?Sized,
    W: CopyIterator<Item = Meta<Vec2, M>> + FromIterator<Meta<Vec2, M>>,
> IntersectTo<Meta<Polygon<V>, M>, GenericPolygon<W, Meta<Vec2, M>>> for Meta<HalfPlane, M>
{
    fn intersect_to(
        &self,
        other: &Meta<Polygon<V>, M>,
    ) -> Option<GenericPolygon<W, Meta<Vec2, M>>> {
        other.intersect_to(self)
    }
}

impl<
    U: CopyIterator<Item = Vec2> + ?Sized,
    V: CopyIterator<Item = Vec2> + ?Sized,
    W: CopyIterator<Item = Vec2> + FromIterator<Vec2>,
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
