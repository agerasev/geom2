mod circle;

pub use circle::Circle;

use glam::Vec2;

pub trait Shape {}

pub struct Intersection {
    pub centroid: Vec2,
    pub area: f32,
}

pub trait Intersect<T: Shape + Intersect<Self> + ?Sized>: Shape {
    fn intersect(&self, other: &T) -> Option<Intersection>;
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

impl Shape for HalfPlane {}
