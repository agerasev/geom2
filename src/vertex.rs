use core::{
    iter::{Copied, Map},
    marker::PhantomData,
};
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

pub trait CopyIterator {
    type Item: Copy;
    type CopyIter<'a>: Iterator<Item = Self::Item> + 'a
    where
        Self: 'a,
        Self::Item: 'a;

    fn iter_copied<'a>(&'a self) -> Self::CopyIter<'a>
    where
        Self::Item: 'a;

    fn map<'a, U, F: Fn(Self::Item) -> U>(&'a self, f: F) -> CopyMap<'a, U, Self, F>
    where
        Self::Item: 'a,
        U: 'a,
    {
        CopyMap {
            iter: self,
            f,
            _ghost: PhantomData,
        }
    }
}

impl<T: Copy, I: ?Sized> CopyIterator for I
where
    for<'a> &'a I: IntoIterator<Item = &'a T>,
{
    type Item = T;
    type CopyIter<'a>
        = Copied<<&'a I as IntoIterator>::IntoIter>
    where
        Self: 'a,
        Self::Item: 'a;

    fn iter_copied<'a>(&'a self) -> Self::CopyIter<'a>
    where
        Self::Item: 'a,
    {
        self.into_iter().copied()
    }
}

pub struct CopyMap<'a, U, I: CopyIterator + ?Sized, F: Fn(I::Item) -> U> {
    iter: &'a I,
    f: F,
    _ghost: PhantomData<U>,
}

impl<'b, U: Copy, I: CopyIterator + ?Sized, F: Fn(I::Item) -> U> CopyIterator
    for CopyMap<'b, U, I, F>
{
    type Item = U;
    type CopyIter<'a>
        = Map<I::CopyIter<'a>, &'a F>
    where
        Self: 'a,
        Self::Item: 'a;

    fn iter_copied<'a>(&'a self) -> Self::CopyIter<'a>
    where
        Self::Item: 'a,
    {
        self.iter.iter_copied().map(&self.f)
    }
}
