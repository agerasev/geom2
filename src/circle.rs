use crate::{
    Arc, ArcVertex, Bounded, CircleSegment, HalfPlane, Integrate, Intersect, IntersectTo, Moment,
    Polygon,
};
use core::f32::consts::PI;
use either::Either;
use glam::Vec2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Bounded for Circle {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        if (self.center - point).length_squared() <= self.radius.powi(2) {
            2 * self.radius.signum() as i32
        } else {
            0
        }
    }
}

impl Integrate for Circle {
    fn moment(&self) -> Moment {
        Moment {
            centroid: self.center,
            area: PI * self.radius.powi(2),
        }
    }
}

impl Intersect<Circle> for HalfPlane {
    type Output = Either<CircleSegment, Circle>;
    fn intersect(&self, circle: &Circle) -> Option<Self::Output> {
        circle.intersect_to(self)
    }
}

impl Intersect<HalfPlane> for Circle {
    type Output = Either<CircleSegment, Circle>;
    fn intersect(&self, other: &HalfPlane) -> Option<Self::Output> {
        let normal = other.normal;
        let apothem = -other.distance(self.center);
        if apothem > self.radius {
            return None;
        }
        if apothem < -self.radius {
            return Some(Either::Right(*self));
        }
        // Half length of the chord
        let h = (self.radius.powi(2) - apothem.powi(2)).sqrt();
        // Midpoint of the chord
        let m = self.center + apothem * normal;
        Some(Either::Left(CircleSegment(Arc {
            points: (m + normal.perp() * h, m - normal.perp() * h),
            sagitta: self.radius - apothem,
        })))
    }
}

impl Intersect<Circle> for Circle {
    type Output = Either<Polygon<[ArcVertex; 2], ArcVertex>, Circle>;
    fn intersect(&self, other: &Circle) -> Option<Self::Output> {
        // Vector pointing from `self.center` to `other.center`
        let rel_pos = other.center - self.center;
        // Distance between the centers of the circles
        let distance = rel_pos.length();
        if distance < self.radius + other.radius {
            if distance > (self.radius - other.radius).abs() {
                let dir = rel_pos / distance;

                // Common chord apothems
                let self_apothem =
                    0.5 * (distance + (self.radius.powi(2) - other.radius.powi(2)) / distance);
                let other_apothem = distance - self_apothem;

                // Half length of the common chord
                let h = (self.radius.powi(2) - self_apothem.powi(2)).sqrt();
                // Midpoint of the common chord
                let m = self.center + dir * self_apothem;

                Some(Either::Left(Polygon::new([
                    ArcVertex {
                        point: m - dir.perp() * h,
                        sagitta: self.radius - self_apothem,
                    },
                    ArcVertex {
                        point: m + dir.perp() * h,
                        sagitta: other.radius - other_apothem,
                    },
                ])))
            } else {
                // One circle is inside another
                if self.radius < other.radius {
                    Some(Either::Right(*self))
                } else {
                    Some(Either::Right(*other))
                }
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains() {
        let circle = Circle {
            center: Vec2::new(0.0, 0.0),
            radius: 1.0,
        };

        assert!(circle.contains(circle.center));

        // Inside points
        assert!(circle.contains(Vec2::new(0.5, 0.0)));
        assert!(circle.contains(Vec2::new(0.0, 0.5)));
        assert!(circle.contains(Vec2::new(0.3, 0.4)));

        // Outside points
        assert!(!circle.contains(Vec2::new(1.5, 0.0)));
        assert!(!circle.contains(Vec2::new(0.0, 1.5)));
        assert!(!circle.contains(Vec2::new(0.9, 0.9)));
    }
}
