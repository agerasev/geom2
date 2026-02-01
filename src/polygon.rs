use glam::Vec2;

use crate::{Clump, Shape};

#[derive(Clone, Copy, Debug)]
pub struct Polygon<V: AsRef<[Vec2]> + ?Sized> {
    pub vertices: V,
}

impl<V: AsRef<[Vec2]>> Polygon<V> {
    pub fn new(vertices: V) -> Self
    where
        V: Sized,
    {
        Self { vertices }
    }
}

impl<V: AsRef<[Vec2]> + ?Sized> Polygon<V> {
    pub fn vertices(&self) -> &[Vec2] {
        self.vertices.as_ref()
    }
}

impl<V: AsRef<[Vec2]> + ?Sized> Shape for Polygon<V> {
    fn clump(&self) -> Clump {
        // Shoelace formula
        let vertices = self.vertices();
        let next_vertices = vertices[1..].iter().copied().chain([vertices[0]]);
        let mut area = 0.0;
        let mut centroid = Vec2::ZERO;
        for (a, b) in (vertices.iter().copied()).zip(next_vertices) {
            let cross = a.perp_dot(b);
            area += cross;
            centroid += (a + b) * cross;
        }
        area = area.abs() * 0.5;
        centroid /= 6.0 * area;
        Clump { area, centroid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square() {
        let square = Polygon::new([
            Vec2::new(0.0, 0.0),
            Vec2::new(3.0, 0.0),
            Vec2::new(3.0, 2.0),
            Vec2::new(0.0, 2.0),
        ]);
        assert_eq!(
            square.clump(),
            Clump {
                area: 6.0,
                centroid: Vec2::new(1.5, 1.0)
            }
        )
    }
}
