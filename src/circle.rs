use crate::{
    Arc, ArcVertex, Closed, DiskSegment, EPS, HalfPlane, Integrable, Intersect, Line, LineSegment,
    Moment, Polygon, impl_approx_eq,
};
use core::{f32::consts::PI, ops::Deref};
use either::Either;
use glam::Vec2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Circle {
    pub fn fill(&self) -> Disk {
        Disk(*self)
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Disk(pub Circle);

impl Disk {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Disk(Circle { center, radius })
    }
    pub fn edge(&self) -> Circle {
        self.0
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

impl_approx_eq!(Circle, f32, center, radius);
impl_approx_eq!(Disk, f32, 0);

/// Order of output points must be the same as in the line
impl Intersect<Line> for Circle {
    type Output = [Vec2; 2];
    fn intersect(&self, line: &Line) -> Option<Self::Output> {
        if line.is_degenerate() {
            return None;
        }
        let plane = HalfPlane::from_edge(*line);
        match self.intersect(&plane)? {
            Either::Left(arc) => Some([arc.points.1, arc.points.0]),
            Either::Right(circle) => {
                if plane.distance(circle.center) > -circle.radius {
                    // Tangent line
                    let point = circle.center
                        + (plane.boundary_point() - circle.center)
                            .project_onto_normalized(plane.normal);
                    Some([point, point])
                } else {
                    None
                }
            }
        }
    }
}

impl Intersect<Circle> for Line {
    type Output = [Vec2; 2];
    fn intersect(&self, circle: &Circle) -> Option<Self::Output> {
        circle.intersect(self)
    }
}

impl Intersect<LineSegment> for Circle {
    type Output = [Option<Vec2>; 2];
    fn intersect(&self, line: &LineSegment) -> Option<Self::Output> {
        let [a, b] = self.intersect(&Line(line.0, line.1))?;
        Some([
            if line.is_between(a) { Some(a) } else { None },
            if line.is_between(b) { Some(b) } else { None },
        ])
    }
}

impl Intersect<Circle> for LineSegment {
    type Output = [Option<Vec2>; 2];
    fn intersect(&self, circle: &Circle) -> Option<Self::Output> {
        circle.intersect(self)
    }
}

impl Intersect<HalfPlane> for Circle {
    type Output = Either<Arc, Circle>;
    fn intersect(&self, plane: &HalfPlane) -> Option<Self::Output> {
        let normal = plane.normal;
        let apothem = plane.distance(self.center);
        if apothem > self.radius {
            return None;
        }
        if apothem <= EPS - self.radius {
            return Some(Either::Right(*self));
        }
        // Half length of the chord
        let half_chord = (self.radius.powi(2) - apothem.powi(2)).sqrt();
        // Midpoint of the chord
        let midpoint = self.center - apothem * normal;
        Some(Either::Left(Arc {
            points: (
                midpoint + normal.perp() * half_chord,
                midpoint - normal.perp() * half_chord,
            ),
            sagitta: self.radius - apothem,
        }))
    }
}

impl Intersect<Circle> for HalfPlane {
    type Output = Either<Arc, Circle>;
    fn intersect(&self, circle: &Circle) -> Option<Self::Output> {
        circle.intersect(self)
    }
}

impl Intersect<HalfPlane> for Disk {
    type Output = Either<DiskSegment, Disk>;
    fn intersect(&self, plane: &HalfPlane) -> Option<Self::Output> {
        Some(match self.edge().intersect(plane)? {
            Either::Left(arc) => Either::Left(DiskSegment(arc)),
            Either::Right(circle) => Either::Right(Disk(circle)),
        })
    }
}

impl Intersect<Disk> for HalfPlane {
    type Output = Either<DiskSegment, Disk>;
    fn intersect(&self, disk: &Disk) -> Option<Self::Output> {
        disk.intersect(self)
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
