use crate::{EPS, Edge, HalfPlane, Intersect, Moments, Shape, Vertex};
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

    fn moments(&self) -> Moments {
        Moments {
            centroid: self.center,
            area: PI * self.radius.powi(2),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CircleOrSegment {
    Circle(Circle),
    Segment(Arc),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Arc {
    pub bounds: (Vec2, Vec2),
    pub sagitta: f32,
}

/// One bound point of arc with the sagitta of the arc to the next bound point.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ArcVertex {
    pub point: Vec2,
    pub sagitta: f32,
}

impl Edge for Arc {
    type Vertex = ArcVertex;
    fn from_vertices(a: &Self::Vertex, b: &Self::Vertex) -> Self {
        Self {
            bounds: (a.point, b.point),
            sagitta: a.sagitta,
        }
    }
}
impl Vertex for ArcVertex {
    type Edge = Arc;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CircleSegment(pub Arc);

impl Shape for CircleSegment {
    fn is_inside(&self, point: Vec2) -> bool {
        unimplemented!()
    }

    fn moments(&self) -> Moments {
        let (a, b) = self.0.bounds;
        let c = 0.5 * (a + b);
        let s = self.0.sagitta;
        if s.abs() < EPS {
            return Moments {
                area: 0.0,
                centroid: c,
            };
        }

        let h = 0.5 * (b - a).length();
        let radius = (h.powi(2) + s.powi(2)) / (2.0 * s);

        let cosine = 1.0 - s / radius;
        let sine = h / radius;
        let (area, offset) = if cosine.abs() < 1.0 - EPS {
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

        let normal = (b - a).perp() / (2.0 * h);
        Moments {
            area: area * radius.powi(2),
            centroid: c + normal * (s + radius * (offset - 1.0)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct CircleSegmentMoments {
    /// Area of the segment
    area: f32,
    /// Offset from the circle center
    offset: f32,
}

impl CircleSegmentMoments {
    /// For given unit circle chord returns segment area and centroid offset.
    ///
    /// Chord is defined via distance from circle center.
    fn new_unit(dist: f32) -> CircleSegmentMoments {
        let cosine = dist.clamp(-1.0, 1.0);
        let sine = (1.0 - cosine.powi(2)).sqrt();
        let (area, offset) = if cosine.abs() < 1.0 - EPS {
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
        CircleSegmentMoments { area, offset }
    }

    fn new(radius: f32, dist: f32) -> CircleSegmentMoments {
        let CircleSegmentMoments { area, offset } = Self::new_unit(dist / radius);
        CircleSegmentMoments {
            area: area * radius.powi(2),
            offset: offset * radius,
        }
    }
}

impl Intersect<Circle, Moments> for HalfPlane {
    fn intersect(&self, circle: &Circle) -> Option<Moments> {
        let plane = self;
        let dist = circle.center.dot(plane.normal) - plane.offset;
        if dist < circle.radius {
            if dist > -circle.radius {
                let segment = CircleSegmentMoments::new(circle.radius, dist);
                Some(Moments {
                    area: segment.area,
                    centroid: circle.center - plane.normal * segment.offset,
                })
            } else {
                Some(Moments {
                    area: PI * circle.radius.powi(2),
                    centroid: circle.center,
                })
            }
        } else {
            None
        }
    }
}

impl Intersect<HalfPlane, Moments> for Circle {
    fn intersect(&self, other: &HalfPlane) -> Option<Moments> {
        other.intersect(self)
    }
}

impl Intersect<Circle, Moments> for Circle {
    fn intersect(&self, other: &Circle) -> Option<Moments> {
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

                let self_segment = CircleSegmentMoments::new(self.radius, self_offset);
                let other_segment = CircleSegmentMoments::new(other.radius, other_offset);

                let area = self_segment.area + other_segment.area;
                Some(Moments {
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
                Some(Moments {
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
        let Moments { area, centroid } = CircleSegment(Arc {
            bounds: (Vec2::new(-EPS, 0.0), Vec2::new(EPS, 0.0)),
            sagitta: 0.0,
        })
        .moments();

        assert_abs_diff_eq!(area, 0.0, epsilon = EPS);
        assert_abs_diff_eq!(centroid, Vec2::ZERO, epsilon = EPS);
    }

    #[test]
    fn full_segment() {
        let Moments { area, centroid } = CircleSegment(Arc {
            bounds: (Vec2::new(-EPS, 0.0), Vec2::new(EPS, 0.0)),
            sagitta: 2.0 * R,
        })
        .moments();

        assert_abs_diff_eq!(area, PI * R.powi(2), epsilon = EPS);
        assert_abs_diff_eq!(centroid, Vec2::new(0.0, R), epsilon = EPS);
    }

    #[test]
    fn half_segment() {
        assert_eq!(
            CircleSegment(Arc {
                bounds: (Vec2::new(-R, 0.0), Vec2::new(R, 0.0)),
                sagitta: R,
            })
            .area(),
            PI * R.powi(2) / 2.0
        );
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
                let y = (1.0 - x.powi(2)).sqrt();
                let ref_segment = CircleSegmentMoments::new(1.0, (1.0 - x) as f32);
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
