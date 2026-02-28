use core::iter::{Copied, Map};
use glam::Vec2;

use crate::Meta;

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

impl<T: Edge, M: Copy> Edge for Meta<T, M> {
    type Vertex = Meta<T::Vertex, M>;
    fn from_vertices(a: &Self::Vertex, b: &Self::Vertex) -> Self {
        Meta::new(T::from_vertices(&a.inner, &b.inner), a.meta)
    }
}

impl<T: Vertex, M: Copy> Vertex for Meta<T, M> {
    type Edge = Meta<T::Edge, M>;
    fn pos(&self) -> Vec2 {
        self.inner.pos()
    }
}

/// A trait for containers that can produce copied iterators.
///
/// This trait is similar to [`IntoIterator`] but guarantees that the iterator
/// yields `Copy` items, and the container itself can be borrowed to produce
/// the iterator multiple times.
pub trait CopyIterator {
    /// The type of items yielded by the iterator.
    ///
    /// This must implement [`Copy`] to allow for efficient iteration
    /// without ownership transfer.
    type Item: Copy;
    /// The iterator type produced by [`iter_copied`](CopyIterator::iter_copied).
    ///
    /// This is a borrowed iterator that yields copied items.
    type CopyIter<'a>: Iterator<Item = Self::Item> + 'a
    where
        Self: 'a,
        Self::Item: 'a;

    /// Create an iterator that yields copied items from the container.
    ///
    /// Unlike [`IntoIterator::into_iter`], this method borrows the container
    /// and can be called multiple times.
    fn iter_copied<'a>(&'a self) -> Self::CopyIter<'a>
    where
        Self::Item: 'a;

    /// Create a reference wrapper that implements [`CopyIterator`].
    ///
    /// This is useful when you need to pass a reference to a container
    /// to a function expecting a [`CopyIterator`].
    fn to_ref<'a>(&'a self) -> CopyRef<'a, Self>
    where
        Self::Item: 'a,
    {
        CopyRef(self)
    }

    /// Apply a transformation function to each item in the iterator.
    ///
    /// Returns a new [`CopyIterator`] that yields the results of applying `f`
    /// to each element of the original iterator.
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

/// A reference wrapper that implements [`CopyIterator`].
///
/// This struct allows you to treat a reference to a container as a [`CopyIterator`].
/// It's typically created by calling [`CopyIterator::to_ref`].
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

/// A mapped iterator that implements [`CopyIterator`].
///
/// This struct is created by the [`CopyIterator::map`] method.
/// It applies a transformation function to each element of the underlying iterator.
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
