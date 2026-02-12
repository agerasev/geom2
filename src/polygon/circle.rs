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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    use core::f32::consts::PI;

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
}
