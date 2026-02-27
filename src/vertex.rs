use core::iter::{Copied, Map};
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

    fn to_ref<'a>(&'a self) -> CopyRef<'a, Self>
    where
        Self::Item: 'a,
    {
        CopyRef(self)
    }

    fn map<'a, U, F: Fn(Self::Item) -> U>(&'a self, f: F) -> CopyMap<'a, Self, F>
    where
        Self::Item: 'a,
        U: 'a,
    {
        CopyMap { iter: self, f }
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

pub struct CopyRef<'a, I: ?Sized>(pub &'a I);

impl<I: CopyIterator + ?Sized> CopyIterator for CopyRef<'_, I> {
    type Item = I::Item;
    type CopyIter<'a>
        = I::CopyIter<'a>
    where
        Self: 'a,
        Self::Item: 'a;

    fn iter_copied<'a>(&'a self) -> Self::CopyIter<'a>
    where
        Self::Item: 'a,
    {
        self.0.iter_copied()
    }
}

pub struct CopyMap<'a, I: ?Sized, F> {
    iter: &'a I,
    f: F,
}

impl<'b, U: Copy, I: CopyIterator + ?Sized, F: Fn(I::Item) -> U> CopyIterator
    for CopyMap<'b, I, F>
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
