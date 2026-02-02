use crate::{Clump, HalfPlane, Intersect, Shape};
use core::f32::consts::PI;
use glam::Vec2;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

impl Shape for Circle {
    fn is_inside(&self, point: Vec2) -> bool {
        (self.center - point).length_squared() <= self.radius.powi(2)
    }

    fn clump(&self) -> Clump {
        Clump {
            centroid: self.center,
            area: PI * self.radius.powi(2),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct CircleSegment {
    /// Area of the segment
    area: f32,
    /// Offset from the circle center
    offset: f32,
}

impl CircleSegment {
    /// For given unit circle chord returns segment area and centroid offset.
    ///
    /// Chord is defined via distance from circle center.
    fn new_unit(dist: f32) -> CircleSegment {
        let cosine = dist.clamp(-1.0, 1.0);
        let sine = (1.0 - cosine.powi(2)).sqrt();
        let (area, offset) = if cosine.abs() < 1.0 - 1e-4 {
            let area = cosine.acos() - cosine * sine;
            (area, (2.0 / 3.0) * sine.powi(3) / area)
        } else {
            // Approximate circle by parabola
            let y = 1.0 - cosine.abs();
            let a = (4.0 / 3.0) * (2.0 * y).sqrt() * y;
            let b = 1.0 - (3.0 / 10.0) * y;
            if cosine > 0.0 {
                (a, b)
            } else {
                (PI - a, -b * a / (PI - a))
            }
        };
        CircleSegment { area, offset }
    }

    fn new(radius: f32, dist: f32) -> CircleSegment {
        let CircleSegment { area, offset } = Self::new_unit(dist / radius);
        CircleSegment {
            area: area * radius.powi(2),
            offset: offset * radius,
        }
    }
}

impl Intersect<Circle> for HalfPlane {
    type Output = Clump;
    fn intersect(&self, circle: &Circle) -> Option<Clump> {
        let plane = self;
        let dist = circle.center.dot(plane.normal) - plane.offset;
        if dist < circle.radius {
            if dist > -circle.radius {
                let segment = CircleSegment::new(circle.radius, dist);
                Some(Clump {
                    area: segment.area,
                    centroid: circle.center - plane.normal * segment.offset,
                })
            } else {
                Some(Clump {
                    area: PI * circle.radius.powi(2),
                    centroid: circle.center,
                })
            }
        } else {
            None
        }
    }
}

impl Intersect<HalfPlane> for Circle {
    type Output = Clump;
    fn intersect(&self, other: &HalfPlane) -> Option<Clump> {
        other.intersect(self)
    }
}

impl Intersect<Circle> for Circle {
    type Output = Clump;
    fn intersect(&self, other: &Circle) -> Option<Clump> {
        // Vector pointing from `self.center` to `other.center`
        let vec = other.center - self.center;
        // Distance between the centers of the circles
        let dist = vec.length();
        if dist < self.radius + other.radius {
            if dist > (self.radius - other.radius).abs() {
                let dir = vec / dist;

                // Common chord offsets
                let self_offset =
                    0.5 * (dist + (self.radius.powi(2) - other.radius.powi(2)) / dist);
                let other_offset = dist - self_offset;

                let self_segment = CircleSegment::new(self.radius, self_offset);
                let other_segment = CircleSegment::new(other.radius, other_offset);

                let area = self_segment.area + other_segment.area;
                Some(Clump {
                    area,
                    centroid: ((self.center + dir * self_segment.offset) * self_segment.area
                        + (other.center - dir * other_segment.offset) * other_segment.area)
                        / area,
                })
            } else {
                let (minr, minc) = if self.radius < other.radius {
                    (self.radius, self.center)
                } else {
                    (other.radius, other.center)
                };
                Some(Clump {
                    area: PI * minr.powi(2),
                    centroid: minc,
                })
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    const R: f32 = 1.234;

    #[test]
    fn empty_segment() {
        assert_eq!(
            CircleSegment::new(R, R),
            CircleSegment {
                area: 0.0,
                offset: R
            }
        );
    }

    #[test]
    fn full_segment() {
        assert_eq!(
            CircleSegment::new(R, -R),
            CircleSegment {
                area: PI * R.powi(2),
                offset: 0.0
            }
        );
    }

    #[test]
    fn half_segment() {
        assert_eq!(CircleSegment::new(R, 0.0).area, PI * R.powi(2) / 2.0);
    }

    #[test]
    fn numerical_segment() {
        let f = |x: f64| 2.0 * (1.0 - (1.0 - x).powi(2)).sqrt();

        let mut x: f64 = 0.0;
        let dx: f64 = 1e-6;

        let (mut area, mut moment) = (0.0, 0.0);

        let check_step = 1e-2;
        let mut last_check = 0.0;
        while x < 2.0 {
            let d_area = 0.5 * (f(x) + f(x + dx)) * dx;
            area += d_area;
            moment += d_area * (x + 0.5 * dx);
            if x >= last_check + check_step {
                last_check = x;
                let ref_segment = CircleSegment::new(1.0, (1.0 - x) as f32);
                assert_abs_diff_eq!(ref_segment.area, area as f32, epsilon = 1e-4);
                assert_abs_diff_eq!(
                    ref_segment.offset,
                    1.0 - (moment / area) as f32,
                    epsilon = 1e-4
                );
            }
            x += dx;
        }
    }
}
