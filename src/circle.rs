use crate::{
    Arc, ArcPolygon, ArcVertex, Closed, DiskSegment, EPS, HalfPlane, Integrable, Intersect, Line,
    LineSegment, Moment, Polygon, impl_approx_eq,
};
use core::{f32::consts::PI, ops::Deref};
use either::Either;
use glam::Vec2;

/// A circle defined by its center and radius.
///
/// ```text
///      ..---..
///    *         *
///  /             \
/// |               |
/// |       +------>|
/// |       c   r   |
///  \             /
///    .         .
///      ``---``
/// ```
///
/// Where `c` is the center and `r` is the radius.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Circle {
    /// Center point of the circle.
    pub center: Vec2,
    /// Radius of the circle.
    pub radius: f32,
}

impl Circle {
    /// Create a filled disk from this circle.
    pub fn fill(&self) -> Disk {
        Disk(*self)
    }
}

/// A filled disk (circle with interior).
///
/// Where `c` is the center and `r` is the radius.
/// The disk includes all points inside the circle.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Disk(pub Circle);

impl Disk {
    /// Create a new disk with the given center and radius.
    pub fn new(center: Vec2, radius: f32) -> Self {
        Disk(Circle { center, radius })
    }

    /// Get the boundary circle of this disk.
    pub fn edge(&self) -> Circle {
        self.0
    }

    /// Approximate the disk as a polygon with `N` vertices.
    ///
    /// Returns an `ArcPolygon` where each edge is a circular arc
    /// approximating a segment of the circle.
    pub fn polygon<const N: usize>(&self) -> ArcPolygon<[ArcVertex; N]> {
        ArcPolygon::<[ArcVertex; N]>::from_circle(self.edge())
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

/// Intersection of a circle with a line.
///
/// Returns the two intersection points as `[Vec2; 2]`, or `None` if the line
/// doesn't intersect the circle. For a tangent line, both points are equal.
///
/// The points are ordered such that when traversing from `line.0` to `line.1`,
/// the first intersection point is encountered before the second.
impl Intersect<Line> for Circle {
    type Output = [Vec2; 2];
    fn intersect(&self, line: &Line) -> Option<Self::Output> {
        if line.is_degenerate() {
            return None;
        }
        let plane = HalfPlane::from_edge(*line);
        match self.intersect(&plane)? {
            Either::Left(arc) => {
                // The arc points are ordered relative to the half-plane normal.
                // Reverse them to match the line direction.
                Some([arc.points.1, arc.points.0])
            }
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
        // Check if the circle is completely inside the half-plane
        // The farthest point of the circle from the plane is at distance (radius + apothem)
        // If this distance is <= EPS, consider the circle fully inside
        if self.radius + apothem <= EPS {
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
