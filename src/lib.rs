mod circle;
mod polygon;

pub use self::{circle::Circle, polygon::Polygon};

use core::f32;
use glam::Vec2;

/// Specific geometric shape.
pub trait Shape {
    fn clump(&self) -> Clump;
    fn area(&self) -> f32 {
        self.clump().area
    }
    fn centroid(&self) -> Vec2 {
        self.clump().centroid
    }
}

/// Abstract shape without an exact form.
#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub struct Clump {
    pub centroid: Vec2,
    pub area: f32,
}

pub trait Intersect<T: Shape + Intersect<Self> + ?Sized>: Shape {
    /// Abstract intersection of two shapes.
    fn intersect(&self, other: &T) -> Option<Clump>;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct HalfPlane {
    /// Normal of the half-plane edge (pointing from occuped space to free space).
    pub normal: Vec2,
    /// Signed distance from origin to the edge of the half-plane.
    ///
    /// If the origin is inside then it is positive, when origin is outside then it is negative.
    pub offset: f32,
}

impl Shape for HalfPlane {
    fn clump(&self) -> Clump {
        Clump {
            centroid: Vec2::INFINITY,
            area: f32::INFINITY,
        }
    }
}
