use glam::Vec2;

use crate::{Arc, ArcVertex, Moments, Polygon, Shape};

impl<V: AsRef<[ArcVertex]> + ?Sized> Shape for Polygon<V, ArcVertex> {
    fn contains(&self, point: Vec2) -> bool {
        let mut winding_number = 0;

        for Arc {
            bounds: (v0, v1),
            sagitta,
        } in self.edges()
        {
            // Test if edge crosses the horizontal line at point.y
            if v0.y <= point.y {
                if v1.y > point.y {
                    // Upward crossing - check if point is left of edge
                    if (v1 - v0).perp_dot(point - v0) > 0.0 {
                        winding_number += 1;
                    }
                }
            } else if v1.y <= point.y {
                // Downward crossing - check if point is right of edge
                if (v1 - v0).perp_dot(point - v0) < 0.0 {
                    winding_number -= 1;
                }
            }
        }

        winding_number != 0
    }

    fn moments(&self) -> Moments {
        // Shoelace formula
        let mut area = 0.0;
        let mut centroid = Vec2::ZERO;
        for Arc {
            bounds: (a, b),
            sagitta,
        } in self.edges()
        {
            let cross = a.perp_dot(b);
            area += cross;
            centroid += (a + b) * cross;
        }
        area = area.abs() * 0.5;
        centroid /= 6.0 * area;
        Moments { area, centroid }
    }
}
