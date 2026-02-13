use crate::{
    ArcVertex, AsIterator, Circle, Closed, Disk, DiskSegment, EPS, Integrable, Intersect,
    IntersectTo, Line, LineSegment, Moment, Polygon,
};
use glam::Vec2;

pub type ArcPolygon<V> = Polygon<V, ArcVertex>;

impl<V: AsIterator<Item = ArcVertex> + ?Sized> ArcPolygon<V> {
    pub fn as_polygon(&self) -> Polygon<impl AsIterator<Item = Vec2>, Vec2> {
        Polygon::new(self.vertices.map(&|arc| &arc.point))
    }
}
impl<V: AsIterator<Item = ArcVertex> + FromIterator<ArcVertex>> ArcPolygon<V> {
    pub fn from_circle(Circle { center, radius }: Circle) -> Self {
        Self::from_iter([
            ArcVertex {
                point: center + Vec2::new(0.0, -radius),
                sagitta: radius,
            },
            ArcVertex {
                point: center + Vec2::new(0.0, radius),
                sagitta: radius,
            },
        ])
    }
}

impl<V: AsIterator<Item = ArcVertex> + ?Sized> Closed for ArcPolygon<V> {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        let mut winding_number = self.as_polygon().winding_number_2(point);

        for arc in self.edges() {
            winding_number += DiskSegment(arc).winding_number_2(point);
        }

        winding_number
    }
}

impl<V: AsIterator<Item = ArcVertex> + ?Sized> Integrable for ArcPolygon<V> {
    fn moment(&self) -> Moment {
        let mut moment = self.as_polygon().moment();

        for arc in self.edges() {
            moment = moment.merge(DiskSegment(arc).moment());
        }

        moment
    }
}

extern crate std;
use std::dbg;

impl<V: AsIterator<Item = Vec2> + ?Sized, W: AsIterator<Item = ArcVertex> + FromIterator<ArcVertex>>
    IntersectTo<Disk, ArcPolygon<W>> for Polygon<V>
{
    fn intersect_to(&self, disk: &Disk) -> Option<ArcPolygon<W>> {
        // Clip vertices
        let mut iter = self.vertices().copied();
        let mut first = None;
        let mut last = None;
        let mut prev = iter.next()?;
        let mut prev_inside = disk.contains(prev);
        let mut iter = iter
            .chain([prev])
            .flat_map(|v| {
                let inside = disk.contains(v);
                dbg!((prev, prev_inside));
                dbg!((v, inside));
                let ret = match (prev_inside, inside) {
                    (true, true) => [
                        Some(ArcVertex {
                            point: prev,
                            sagitta: 0.0,
                        }),
                        None,
                    ],
                    (true, false) => {
                        last = Some(disk.edge().intersect(&Line(prev, v)).unwrap()[1]);
                        [
                            Some(ArcVertex {
                                point: prev,
                                sagitta: 0.0,
                            }),
                            None,
                        ]
                    }
                    (false, true) => {
                        let clip = disk.edge().intersect(&Line(prev, v)).unwrap()[0];
                        [
                            if let Some(last) = last {
                                Some(ArcVertex {
                                    point: last,
                                    sagitta: disk.radius
                                        - Line(last, clip).signed_distance(disk.center),
                                })
                            } else {
                                if first.is_none() {
                                    first = Some(clip);
                                }
                                None
                            },
                            Some(ArcVertex {
                                point: clip,
                                sagitta: 0.0,
                            }),
                        ]
                    }
                    (false, false) => match disk.edge().intersect(&LineSegment(prev, v)) {
                        Some([Some(a), Some(b)]) => {
                            dbg!((a, b));
                            let ret = [
                                if let Some(last) = last {
                                    Some(ArcVertex {
                                        point: last,
                                        sagitta: disk.radius
                                            - Line(last, a).signed_distance(disk.center),
                                    })
                                } else {
                                    if first.is_none() {
                                        first = Some(a);
                                    }
                                    None
                                },
                                Some(ArcVertex {
                                    point: a,
                                    sagitta: 0.0,
                                }),
                            ];
                            last = Some(b);
                            ret
                        }
                        _ => [None, None],
                    },
                };
                dbg!(ret);
                prev_inside = inside;
                prev = v;
                ret
            })
            .flatten()
            // .chain(first.zip(last).map(|(a, b)| ArcVertex {
            //     point: b,
            //     sagitta: disk.radius - Line(b, a).signed_distance(disk.center),
            // }))
            ;

        // Deduplicate vertices
        let mut prev = iter.next()?;
        let iter = iter.chain([prev]).filter_map(|v| {
            let ret = if (prev.point - v.point).abs().max_element() > EPS {
                Some(prev)
            } else {
                None
            };
            prev = v;
            ret
        });

        let result = ArcPolygon::<W>::from_iter(iter);
        if !result.is_empty() {
            Some(result)
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
