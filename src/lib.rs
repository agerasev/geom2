mod circle;
mod half_plane;
mod polygon;

pub use self::{circle::Circle, half_plane::HalfPlane, polygon::Polygon};

use core::{cmp::Ordering, f32};
use glam::Vec2;

/// Specific geometric shape.
pub trait Shape {
    // fn bounding_box(&self) -> (Vec2, Vec2);

    fn locate(&self, point: Vec2) -> Location;

    fn clump(&self) -> Clump;
    fn area(&self) -> f32 {
        self.clump().area
    }
    fn centroid(&self) -> Vec2 {
        self.clump().centroid
    }
}

/// Point location relative to the shape
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Location {
    Inside,
    AtEdge,
    Outside,
}

impl Location {
    pub fn from_distance(distance: f32) -> Self {
        match distance.partial_cmp(&0.0).unwrap() {
            Ordering::Less => Location::Inside,
            Ordering::Equal => Location::AtEdge,
            Ordering::Greater => Location::Outside,
        }
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
