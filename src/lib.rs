mod circle;
mod line;
mod plane;
//mod polygon;

pub use self::{
    circle::Circle,
    line::{Line, LineSegment},
    plane::HalfPlane,
    //polygon::Polygon,
};

use core::f32;
use glam::Vec2;

/// Specific geometric shape.
pub trait Shape {
    // fn bounding_box(&self) -> (Vec2, Vec2);

    /// Check that the `point` is inside the shape.
    ///
    /// Shape is considered to be closed rather than open.
    /// That means the boundary points is inside the shape.
    fn is_inside(&self, point: Vec2) -> bool;

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

pub trait Intersect<T: Intersect<Self> + ?Sized> {
    type Output: Sized;
    /// Abstract intersection of two figures.
    fn intersect(&self, other: &T) -> Option<Self::Output>;
}
