use crate::{Closed, Disk, EPS, Edge, Integrable, LineSegment, Moment, Vertex, impl_approx_eq};
use core::{f32::consts::PI, ops::Deref};
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
    /// Get the chord connecting the endpoints of this arc.
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
///
/// The disk segment is the region between the arc and its chord.
/// It includes all points inside the circle sector defined by the arc.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct DiskSegment(pub Arc);

impl Deref for DiskSegment {
    type Target = Arc;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Closed for DiskSegment {
    fn winding_number_2(&self, point: Vec2) -> i32 {
        let (a, b) = self.0.points;
        let midpoint = 0.5 * (a + b);
        let sagitta = self.0.sagitta.abs();
        if sagitta < EPS {
            return 0;
        }

        let half_chord = 0.5 * (b - a).length();
        let radius = (half_chord.powi(2) + sagitta.powi(2)) / (2.0 * sagitta);
        let normal = -(b - a).perp() / (2.0 * half_chord) * self.0.sagitta.signum();
        let center = midpoint + normal * (sagitta - radius);

        if Disk::new(center, radius).contains(point) && (point - midpoint).dot(normal) > 0.0 {
            2 * self.0.sagitta.signum() as i32
        } else {
            0
        }
    }
}

/// Maximum ratio between sagitta and radius where the circle arc can be approximated by the parabola.
const APPROX_CIRCLE: f32 = 1e-4;

impl Integrable for DiskSegment {
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

impl_approx_eq!(Arc, f32, points.0, points.1, sagitta);
impl_approx_eq!(ArcVertex, f32, point, sagitta);
impl_approx_eq!(DiskSegment, f32, points.0, points.1, sagitta);
