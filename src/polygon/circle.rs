use crate::{
    ArcVertex, AsIterator, Circle, Closed, Disk, DiskSegment, EPS, Integrable, Intersect,
    IntersectTo, Line, LineSegment, Moment, Polygon,
};
use core::{array::from_fn, f32::consts::PI};
use genawaiter::{stack::let_gen, yield_};
use glam::Vec2;

pub type ArcPolygon<V> = Polygon<V, ArcVertex>;

impl<V: AsIterator<Item = ArcVertex> + ?Sized> ArcPolygon<V> {
    pub fn frame(&self) -> Polygon<impl AsIterator<Item = Vec2>, Vec2> {
        Polygon::new(self.vertices.map(&|arc| &arc.point))
    }
}
impl<const N: usize> ArcPolygon<[ArcVertex; N]> {
    pub fn from_circle(Circle { center, radius }: Circle) -> Self {
        Self::new(from_fn(|i| ArcVertex {
            point: center + radius * Vec2::from_angle(2.0 * PI * i as f32 / N as f32),
            sagitta: radius * (1.0 - (PI / N as f32).cos()),
        }))
    }
}

impl<V: AsIterator<Item = ArcVertex> + ?Sized> Closed for ArcPolygon<V> {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        let mut winding_number = self.frame().winding_number_2(point);

        for arc in self.edges() {
            winding_number += DiskSegment(arc).winding_number_2(point);
        }

        winding_number
    }
}

impl<V: AsIterator<Item = ArcVertex> + ?Sized> Integrable for ArcPolygon<V> {
    fn moment(&self) -> Moment {
        let mut moment = self.frame().moment();

        for arc in self.edges() {
            moment = moment.merge(DiskSegment(arc).moment());
        }

        moment
    }
}

impl<V: AsIterator<Item = Vec2> + ?Sized, W: AsIterator<Item = ArcVertex> + FromIterator<ArcVertex>>
    IntersectTo<Disk, ArcPolygon<W>> for Polygon<V>
{
    fn intersect_to(&self, disk: &Disk) -> Option<ArcPolygon<W>> {
        // Clip vertices
        let_gen!(gen_, {
            let mut iter = self.vertices();
            let mut first = None;
            let mut last = None;
            let mut prev = match iter.next() {
                Some(x) => x,
                None => return,
            };
            let mut prev_inside = disk.contains(prev);
            for v in iter.chain([prev]) {
                let inside = disk.contains(v);
                match (prev_inside, inside) {
                    (true, true) => {
                        yield_!(ArcVertex {
                            point: prev,
                            sagitta: 0.0,
                        });
                    }
                    (true, false) => {
                        last = Some(disk.edge().intersect(&Line(prev, v)).unwrap_or([prev, v])[1]);
                        yield_!(ArcVertex {
                            point: prev,
                            sagitta: 0.0,
                        });
                    }
                    (false, true) => {
                        let clip = disk.edge().intersect(&Line(prev, v)).unwrap_or([prev, v])[0];
                        if let Some(last) = last {
                            yield_!(ArcVertex {
                                point: last,
                                sagitta: disk.radius
                                    - Line(last, clip).signed_distance(disk.center),
                            })
                        } else {
                            if first.is_none() {
                                first = Some(clip);
                            }
                        }
                        yield_!(ArcVertex {
                            point: clip,
                            sagitta: 0.0,
                        });
                    }
                    (false, false) => match disk.edge().intersect(&LineSegment(prev, v)) {
                        Some([Some(a), Some(b)]) => {
                            if let Some(last) = last {
                                yield_!(ArcVertex {
                                    point: last,
                                    sagitta: disk.radius
                                        - Line(last, a).signed_distance(disk.center),
                                });
                            } else {
                                if first.is_none() {
                                    first = Some(a);
                                }
                            }
                            yield_!(ArcVertex {
                                point: a,
                                sagitta: 0.0,
                            });
                            last = Some(b);
                        }
                        _ => {}
                    },
                };
                prev_inside = inside;
                prev = v;
            }
            if let (Some(a), Some(b)) = (first, last) {
                yield_!(ArcVertex {
                    point: b,
                    sagitta: disk.radius - Line(b, a).signed_distance(disk.center),
                });
            }
        });
        let mut iter = gen_.into_iter();

        if let Some(mut prev) = iter.next() {
            // Deduplicate vertices
            let iter = iter.chain([prev]).filter_map(|v| {
                let ret = if (prev.point - v.point).abs().max_element() > EPS {
                    Some(prev)
                } else {
                    None
                };
                prev = v;
                ret
            });
            Some(ArcPolygon::<W>::from_iter(iter))
        } else if self.contains(disk.center) {
            Some(ArcPolygon::<W>::from_iter(disk.polygon::<2>().vertices()))
        } else {
            None
        }
    }
}

impl<V: AsIterator<Item = Vec2> + ?Sized, W: AsIterator<Item = ArcVertex> + FromIterator<ArcVertex>>
    IntersectTo<Polygon<V>, ArcPolygon<W>> for Disk
{
    fn intersect_to(&self, other: &Polygon<V>) -> Option<ArcPolygon<W>> {
        other.intersect_to(self)
    }
}
