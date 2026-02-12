use crate::{EPS, Edge, Intersect, Vertex};
use glam::Vec2;

/// Infinite line defined by two points lying on it.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Line(pub Vec2, pub Vec2);

/// Line segment bounded by two points.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct LineSegment(pub Vec2, pub Vec2);

impl Line {
    pub fn is_degenerate(&self) -> bool {
        (self.1 - self.0).abs().max_element() < EPS
    }

    /// Check that point is within EPS-neighbourhood of the line.
    pub fn is_near(&self, point: Vec2) -> bool {
        let r = self.1 - self.0;

        // Check if `self` is degenerate
        if r.abs().max_element() < EPS {
            return (point - self.0).abs().max_element() < EPS;
        }

        // Check collinearity using cross product
        let cross = r.perp_dot(point - self.0);
        cross.abs() < EPS
    }
}

impl LineSegment {
    /// Returns the line containing this segment
    pub fn to_line(&self) -> Line {
        Line(self.0, self.1)
    }

    /// Returns true if this segment has zero length
    pub fn is_degenerate(&self) -> bool {
        Line(self.0, self.1).is_degenerate()
    }

    /// Checks is a point is within EPS-neighbourhood of the segment
    pub fn is_near(&self, point: Vec2) -> bool {
        let r = self.1 - self.0;

        // Check if `self` is degenerate
        if r.abs().max_element() < EPS {
            return (point - self.0).abs().max_element() < EPS;
        }

        // Check collinearity using cross product
        let cross = r.perp_dot(point - self.0);
        if cross.abs() > EPS {
            return false;
        }

        // Check that point lies between endpoints using dot product
        let dot = (point - self.0).dot(r);
        dot >= -EPS && dot <= r.length_squared() + EPS
    }
}

impl Edge for LineSegment {
    type Vertex = Vec2;
    fn from_vertices(a: &Self::Vertex, b: &Self::Vertex) -> Self {
        LineSegment(*a, *b)
    }
}
impl Vertex for Vec2 {
    type Edge = LineSegment;
}

impl Intersect<Line> for Line {
    type Output = Vec2;
    fn intersect(&self, other: &Line) -> Option<Vec2> {
        let p = self.0;
        let q = other.0;
        let r = self.1 - self.0;
        let s = other.1 - other.0;
        let pq = q - p;

        let den = r.perp_dot(s);
        let pqr = pq.perp_dot(r);
        let pqs = pq.perp_dot(s);

        if den.abs() > EPS {
            Some(Vec2::lerp(self.0, self.1, pqs / den))
        } else {
            match (r.abs().max_element() > EPS, s.abs().max_element() > EPS) {
                (true, true) => {
                    // Lines are parallel
                    if pqs.abs() < EPS {
                        // Lines are coincident. Return any point on the line
                        Some(p)
                    } else {
                        // Lines are parallel but not coincident
                        None
                    }
                }
                (false, true) => {
                    // Line `self` is degenerate
                    if pqs.abs() < EPS { Some(p) } else { None }
                }
                (true, false) => {
                    // Line `other` is degenerate
                    if pqr.abs() < EPS { Some(q) } else { None }
                }
                (false, false) => {
                    // Both lines are degenerate
                    if pq.abs().max_element() < EPS {
                        Some(p)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

impl Intersect<Line> for LineSegment {
    type Output = Vec2;
    fn intersect(&self, other: &Line) -> Option<Vec2> {
        let p = self.0;
        let q = other.0;
        let r = self.1 - self.0;
        let s = other.1 - other.0;
        let pq = q - p;

        let den = r.perp_dot(s);
        let pqr = pq.perp_dot(r);
        let pqs = pq.perp_dot(s);

        if den.abs() > EPS {
            let u = pqs / den;
            if (-EPS..=(1.0 + EPS)).contains(&u) {
                Some(Vec2::lerp(self.0, self.1, u))
            } else {
                None
            }
        } else {
            match (r.abs().max_element() > EPS, s.abs().max_element() > EPS) {
                (true, true) => {
                    // Segment line is parallel to the other line
                    if pqs.abs() < EPS {
                        // Segment overlaps with line. Return the center of the segment
                        Some(p + 0.5 * r)
                    } else {
                        None
                    }
                }
                (false, true) => {
                    // Segment `self` is degenerate
                    if pqs.abs() < EPS { Some(p) } else { None }
                }
                (true, false) => {
                    // Line `other` is degenerate
                    let u = pq.dot(r) / r.length_squared();
                    if pqr.abs() < EPS && (-EPS..=(1.0 + EPS)).contains(&u) {
                        Some(q)
                    } else {
                        None
                    }
                }
                (false, false) => {
                    // Both are degenerate
                    if pq.abs().max_element() < EPS {
                        Some(p)
                    } else {
                        None
                    }
                }
            }
        }
    }
}

impl Intersect<LineSegment> for Line {
    type Output = Vec2;
    fn intersect(&self, other: &LineSegment) -> Option<Vec2> {
        other.intersect(self)
    }
}

impl Intersect<LineSegment> for LineSegment {
    type Output = Vec2;
    fn intersect(&self, other: &LineSegment) -> Option<Vec2> {
        let p = self.0;
        let q = other.0;
        let r = self.1 - self.0;
        let s = other.1 - other.0;
        let pq = q - p;

        let den = r.perp_dot(s);
        let pqr = pq.perp_dot(r);
        let pqs = pq.perp_dot(s);

        if den.abs() > EPS {
            let u = pqs / den;
            let v = pqr / den;
            if (-EPS..=(1.0 + EPS)).contains(&u) && (-EPS..=(1.0 + EPS)).contains(&v) {
                Some(Vec2::lerp(self.0, self.1, u))
            } else {
                None
            }
        } else {
            match (r.abs().max_element() > EPS, s.abs().max_element() > EPS) {
                (true, true) => {
                    // Segments are parallel
                    if pqr.abs() < EPS {
                        // Segments are collinear
                        // Check for overlap
                        let t0 = pq.dot(r) / r.length_squared();
                        let t1 = (pq + s).dot(r) / r.length_squared();

                        let t_min = t0.min(t1);
                        let t_max = t0.max(t1);

                        if t_max < -EPS || t_min > 1.0 + EPS {
                            // No overlap
                            None
                        } else {
                            // Segments overlap
                            // Return the midpoint of the overlapping region
                            let overlap_start = t_min.max(0.0);
                            let overlap_end = t_max.min(1.0);
                            let t_mid = (overlap_start + overlap_end) * 0.5;
                            Some(self.0 + r * t_mid)
                        }
                    } else {
                        // Parallel but not collinear
                        None
                    }
                }
                (false, true) => {
                    // Segment `self` is degenerate
                    let v = -pq.dot(s) / s.length_squared();
                    if pqs.abs() < EPS && (-EPS..=(1.0 + EPS)).contains(&v) {
                        Some(p)
                    } else {
                        None
                    }
                }
                (true, false) => {
                    // Segment `other` is degenerate
                    let u = pq.dot(r) / r.length_squared();
                    if pqr.abs() < EPS && (-EPS..=(1.0 + EPS)).contains(&u) {
                        Some(q)
                    } else {
                        None
                    }
                }
                (false, false) => {
                    // Both segments are degenerate
                    if pq.abs().max_element() < EPS {
                        Some(p)
                    } else {
                        None
                    }
                }
            }
        }
    }
}
