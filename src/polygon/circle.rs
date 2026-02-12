use crate::{ArcVertex, AsIterator, Bounded, DiskSegment, Integrate, Moment, Polygon};
use glam::Vec2;

impl<V: AsIterator<Item = ArcVertex> + ?Sized> Polygon<V, ArcVertex> {
    pub fn as_polygon(&self) -> Polygon<impl AsIterator<Item = Vec2>, Vec2> {
        Polygon::new(self.vertices.map(&|arc| &arc.point))
    }
}

impl<V: AsIterator<Item = ArcVertex> + ?Sized> Bounded for Polygon<V, ArcVertex> {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        let mut winding_number = self.as_polygon().winding_number_2(point);

        for arc in self.edges() {
            winding_number += DiskSegment(arc).winding_number_2(point);
        }

        winding_number
    }
}

impl<V: AsIterator<Item = ArcVertex> + ?Sized> Integrate for Polygon<V, ArcVertex> {
    fn moment(&self) -> Moment {
        let mut moment = self.as_polygon().moment();

        for arc in self.edges() {
            moment = moment.merge(DiskSegment(arc).moment());
        }

        moment
    }
}
