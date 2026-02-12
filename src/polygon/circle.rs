use crate::{
    ArcVertex, AsIterator, Circle, Closed, Disk, DiskSegment, Integrable, Intersect, IntersectTo,
    Line, Moment, Polygon,
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
        Self::from_iter(
            [
                ArcVertex {
                    point: center + Vec2::new(0.0, -radius),
                    sagitta: radius,
                },
                ArcVertex {
                    point: center + Vec2::new(0.0, radius),
                    sagitta: radius,
                },
            ]
            .into_iter(),
        )
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

impl<V: AsIterator<Item = Vec2> + ?Sized, W: AsIterator<Item = ArcVertex> + FromIterator<ArcVertex>>
    IntersectTo<Disk, ArcPolygon<W>> for Polygon<V>
{
    fn intersect_to(&self, disk: &Disk) -> Option<ArcPolygon<W>> {
        let mut last = Vec2::ZERO;
        let (n, mut prev) = match self
            .vertices()
            .copied()
            .enumerate()
            .find(|(_, v)| disk.contains(*v))
        {
            Some(x) => x,
            None => {
                return if self.contains(disk.center) {
                    Some(ArcPolygon::<W>::from_circle(**disk))
                } else {
                    None
                };
            }
        };
        let mut prev_inside = true;
        let clip_iter = (self.vertices().skip(n + 1))
            .chain(self.vertices().take(n))
            .copied()
            .flat_map(|v| {
                let inside = disk.contains(v);
                let ret = match (prev_inside, inside) {
                    (true, true) => [
                        Some(ArcVertex {
                            point: prev,
                            sagitta: 0.0,
                        }),
                        None,
                    ],
                    (true, false) => {
                        last = disk.edge().intersect_to(&Line(prev, v)).unwrap()[1];
                        [
                            Some(ArcVertex {
                                point: prev,
                                sagitta: 0.0,
                            }),
                            None,
                        ]
                    }
                    (false, true) => {
                        let clip = disk.edge().intersect_to(&Line(prev, v)).unwrap()[0];
                        [
                            Some(ArcVertex {
                                point: last,
                                sagitta: todo!(),
                            }),
                            Some(ArcVertex {
                                point: clip,
                                sagitta: 0.0,
                            }),
                        ]
                    }
                    (false, false) => match disk.edge().intersect(&Line(prev, v)) {
                        Some([a, b]) => {
                            let ret = [
                                Some(ArcVertex {
                                    point: last,
                                    sagitta: todo!(),
                                }),
                                Some(ArcVertex {
                                    point: a,
                                    sagitta: 0.0,
                                }),
                            ];
                            last = b;
                            ret
                        }
                        None => [None, None],
                    },
                };
                prev_inside = inside;
                prev = v;
                ret
            })
            .flatten();
        let result = ArcPolygon::<W>::from_iter(clip_iter);
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
