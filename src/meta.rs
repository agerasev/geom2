use crate::CopyIterator;
use core::{
    iter::Map,
    ops::{Deref, DerefMut},
};

/// Wrapper around geometry with an associated metadata.
///
/// This struct attaches arbitrary metadata `M` to a geometric shape `T`.
/// The metadata must be `Copy` and is stored alongside the shape.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Meta<T: ?Sized, M: Copy> {
    /// The metadata associated with the geometry.
    pub meta: M,
    /// The wrapped geometric shape.
    pub inner: T,
}

impl<T, M: Copy> Meta<T, M> {
    /// Create a new `Meta` wrapper with the given geometry and metadata.
    pub fn new(inner: T, metadata: M) -> Self {
        Self {
            inner,
            meta: metadata,
        }
    }
}

impl<T: ?Sized, M: Copy> Deref for Meta<T, M> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: ?Sized, M: Copy> DerefMut for Meta<T, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// A wrapper that removes metadata from geometry.
///
/// This struct is used to convert between metadata-wrapped geometry
/// and plain geometry. It implements [`CopyIterator`] for containers
/// of metadata-wrapped items, removing the metadata in the process.
pub struct Unmeta<I: ?Sized>(pub I);

impl<I: CopyIterator + ?Sized> CopyIterator for Unmeta<I> {
    type Item = Meta<I::Item, ()>;
    type CopyIter<'a>
        = Map<I::CopyIter<'a>, fn(I::Item) -> Meta<I::Item, ()>>
    where
        Self: 'a,
        Self::Item: 'a;

    fn iter_copied<'a>(&'a self) -> Self::CopyIter<'a>
    where
        Self::Item: 'a,
    {
        self.0.iter_copied().map(|x| Meta::new(x, ()))
    }
}

impl<T, I: FromIterator<T>> FromIterator<Meta<T, ()>> for Unmeta<I> {
    fn from_iter<J: IntoIterator<Item = Meta<T, ()>>>(iter: J) -> Self {
        Self(I::from_iter(iter.into_iter().map(|x| x.inner)))
    }
}
