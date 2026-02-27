use core::{iter::Map, marker::PhantomData};
use glam::Vec2;

/// Trait for edges of a polygon.
pub trait Edge: Copy {
    /// The vertex type for this edge.
    type Vertex: Vertex<Edge = Self>;

    /// Create an edge from two vertices.
    fn from_vertices(a: &Self::Vertex, b: &Self::Vertex) -> Self;
}

/// Trait for vertices of a polygon.
pub trait Vertex: Copy {
    /// The edge type for this vertex.
    type Edge: Edge<Vertex = Self>;

    /// Get coordinates of the vertex.
    fn pos(&self) -> Vec2;
}

pub trait AsIterator {
    type Item;
    type RefIter<'a>: Iterator<Item = &'a Self::Item>
    where
        Self: 'a;

    fn iter(&self) -> Self::RefIter<'_>;

    fn map<'a, U, F: Fn(&Self::Item) -> &U>(&'a self, f: &'a F) -> AsMap<'a, U, Self, F> {
        AsMap {
            iter: self,
            f,
            _ghost: PhantomData,
        }
    }
}

impl<T, I: ?Sized> AsIterator for I
where
    for<'a> &'a I: IntoIterator<Item = &'a T>,
{
    type Item = T;
    type RefIter<'a>
        = <&'a I as IntoIterator>::IntoIter
    where
        Self: 'a;

    fn iter(&self) -> Self::RefIter<'_> {
        self.into_iter()
    }
}

pub struct AsMap<'a, U, I: AsIterator + ?Sized, F: Fn(&I::Item) -> &U> {
    iter: &'a I,
    f: &'a F,
    _ghost: PhantomData<U>,
}

impl<'b, U, I: AsIterator + ?Sized, F: Fn(&I::Item) -> &U> AsIterator for AsMap<'b, U, I, F> {
    type Item = U;
    type RefIter<'a>
        = Map<I::RefIter<'a>, &'a F>
    where
        Self: 'a;

    fn iter(&self) -> Self::RefIter<'_> {
        self.iter.iter().map(self.f)
    }
}
