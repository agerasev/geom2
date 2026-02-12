use core::f32::consts::PI;

use crate::{Bounded, Disk, EPS, Edge, Integrate, LineSegment, Moment, Vertex};
use glam::Vec2;

/// Circular arc.
///
/// Defined by:
/// + `points` — vertices on its ends,
/// + `sagitta` — distance from midpoint of the chord to the midpoint of the arc.
///
/// ```text
///      ..-+-..
///    *    |    * arc
///  /      | s    \
/// |       |       |
/// +-------+-------+
/// b1    chord     b0
/// ```
///
/// Where `(b0, b1)` — end points, `s` - sagitta.
///
/// Sagitta is signed. When looking from the first end to the second one,
/// the positive sagitta will make the arc to the right side of the chord,
/// while the negative sagitta — to the left side.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Arc {
    pub points: (Vec2, Vec2),
    pub sagitta: f32,
}

impl Arc {
    pub fn chord(&self) -> LineSegment {
        LineSegment(self.points.0, self.points.1)
    }
}

/// Start point of an [`Arc`] with its sagitta.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ArcVertex {
    pub point: Vec2,
    pub sagitta: f32,
}

impl Edge for Arc {
    type Vertex = ArcVertex;
    fn from_vertices(a: &Self::Vertex, b: &Self::Vertex) -> Self {
        Self {
            points: (a.point, b.point),
            sagitta: a.sagitta,
        }
    }
}
impl Vertex for ArcVertex {
    type Edge = Arc;
}

/// Disk segment bounded by an arc and its chord.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DiskSegment(pub Arc);

impl Bounded for DiskSegment {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        let (a, b) = self.0.points;
        let c = 0.5 * (a + b);
        let s = self.0.sagitta.abs();
        if s < EPS {
            return 0;
        }

        let h = 0.5 * (b - a).length();
        let radius = (h.powi(2) + s.powi(2)) / (2.0 * s);
        let normal = -(b - a).perp() / (2.0 * h) * self.0.sagitta.signum();
        let center = c + normal * (s - radius);

        if Disk::new(center, radius).contains(point) && (point - c).dot(normal) > 0.0 {
            2 * self.0.sagitta.signum() as i32
        } else {
            0
        }
    }
}

/// Maximum ratio between sagitta and radius where the circle arc can be approximated by the parabola.
const APPROX_CIRCLE: f32 = 1e-4;

extern crate std;

impl Integrate for DiskSegment {
    fn moment(&self) -> Moment {
        let (a, b) = self.0.points;
        let c = 0.5 * (a + b);
        let s = self.0.sagitta.abs();
        if s < EPS {
            return Moment {
                area: 0.0,
                centroid: c,
            };
        }

        let h = 0.5 * (b - a).length();
        let radius = (h.powi(2) + s.powi(2)) / (2.0 * s);

        let cosine = 1.0 - s / radius;
        let sine = h / radius;
        let (area, offset) = if s > APPROX_CIRCLE * radius {
            let area = cosine.acos() - cosine * sine;
            (area, (2.0 / 3.0) * sine.powi(3) / area)
        } else {
            // Approximate circle by parabola
            let y = 1.0 - cosine.abs();
            let area = (4.0 / 3.0) * (2.0 * y).sqrt() * y;
            let offset = 1.0 - (3.0 / 10.0) * y;
            if cosine > 0.0 {
                (area, offset)
            } else {
                (PI - area, -offset * area / (PI - area))
            }
        };

        let normal = -(b - a).perp() / (2.0 * h) * self.0.sagitta.signum();
        Moment {
            area: area * radius.powi(2),
            centroid: c + normal * (s + radius * (offset - 1.0)),
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
        let Moment { area, centroid } = DiskSegment(Arc {
            points: (Vec2::new(EPS, 0.0), Vec2::new(-EPS, 0.0)),
            sagitta: 0.0,
        })
        .moment();

        assert_abs_diff_eq!(area, 0.0, epsilon = EPS);
        assert_abs_diff_eq!(centroid, Vec2::ZERO, epsilon = EPS);
    }

    #[test]
    fn full_segment() {
        let Moment { area, centroid } = DiskSegment(Arc {
            points: (Vec2::new(EPS, 0.0), Vec2::new(-EPS, 0.0)),
            sagitta: 2.0 * R,
        })
        .moment();

        assert_abs_diff_eq!(area, PI * R.powi(2), epsilon = EPS);
        assert_abs_diff_eq!(centroid, Vec2::new(0.0, R), epsilon = EPS);
    }

    #[test]
    fn half_segment() {
        assert_eq!(
            DiskSegment(Arc {
                points: (Vec2::new(R, 0.0), Vec2::new(-R, 0.0)),
                sagitta: R,
            })
            .area(),
            PI * R.powi(2) / 2.0
        );
    }

    #[test]
    fn segment_contains() {
        let segment = DiskSegment(Arc {
            points: (Vec2::new(4.0, 1.0), Vec2::new(1.0, 1.0)),
            sagitta: 4.0,
        });

        assert!(!segment.contains(Vec2::new(2.5, 5.01)));
        assert!(segment.contains(Vec2::new(2.5, 4.99)));

        assert!(segment.contains(Vec2::new(2.5, 1.01)));
        assert!(!segment.contains(Vec2::new(2.5, 0.99)));
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
                let y = (1.0 - (1.0 - x).powi(2)).sqrt();
                let ref_segment = DiskSegment(Arc {
                    points: (
                        Vec2::new(x as f32, y as f32),
                        Vec2::new(x as f32, -y as f32),
                    ),
                    sagitta: x as f32,
                });
                assert_abs_diff_eq!(ref_segment.area(), area as f32, epsilon = 1e-4);
                assert_abs_diff_eq!(
                    ref_segment.centroid(),
                    Vec2::new((moment / area) as f32, 0.0),
                    epsilon = 1e-4
                );
            }
            x += dx;
        }
    }
}
