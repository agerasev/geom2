use crate::{
    Arc, ArcVertex, Closed, DiskSegment, HalfPlane, Integrable, Intersect, Moment, Polygon,
};
use core::{f32::consts::PI, ops::Deref};
use either::Either;
use glam::Vec2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Disk(pub Circle);

impl Disk {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Disk(Circle { center, radius })
    }
}

impl Deref for Disk {
    type Target = Circle;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Closed for Disk {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        if (self.center - point).length_squared() <= self.radius.powi(2) {
            2 * self.radius.signum() as i32
        } else {
            0
        }
    }
}

impl Integrable for Disk {
    fn moment(&self) -> Moment {
        Moment {
            centroid: self.center,
            area: PI * self.radius.powi(2),
        }
    }
}

impl Intersect<Disk> for HalfPlane {
    type Output = Either<DiskSegment, Disk>;
    fn intersect(&self, disk: &Disk) -> Option<Self::Output> {
        disk.intersect(self)
    }
}

impl Intersect<HalfPlane> for Disk {
    type Output = Either<DiskSegment, Disk>;
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
        Some(Either::Left(DiskSegment(Arc {
            points: (m + normal.perp() * h, m - normal.perp() * h),
            sagitta: self.radius - apothem,
        })))
    }
}

impl Intersect<Disk> for Disk {
    type Output = Either<Polygon<[ArcVertex; 2], ArcVertex>, Disk>;
    fn intersect(&self, other: &Disk) -> Option<Self::Output> {
        // Vector pointing from `self.center` to `other.center`
        let rel_pos = other.center - self.center;
        // Distance between the centers of the circles
        let distance = rel_pos.length();
        if distance <= self.radius + other.radius {
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
