pub mod circle;
pub mod line;

use crate::AsIterator;
use core::{
    fmt::{self, Debug, Formatter},
    iter::Copied,
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

/// A polygon defined by a sequence of vertices.
///
/// The polygon can have any vertex type `T` that implements [`Vertex`],
/// and the vertices can be stored in any container `V` that implements [`AsIterator`].
///
/// ```text
///     v4 +-----+ v4
///       /       \
///      /         \
///  v5 +           + v3
///      \         /
///       \       /
///     v0 +-----+ v2
/// ```
///
/// Vertices are connected in order: v0 -> v1 -> v2 -> v3 -> v4 -> v5 -> v0.
#[derive(Clone, Copy)]
pub struct Polygon<V: AsIterator<Item = T> + ?Sized, T: Vertex = Vec2> {
    _ghost: PhantomData<T>,
    /// The vertices of the polygon.
    pub vertices: V,
}

impl<T: Vertex, V: AsIterator<Item = T>> Polygon<V, T> {
    /// Create a new polygon from a sequence of vertices.
    pub fn new(vertices: V) -> Self {
        Self {
            vertices,
            _ghost: PhantomData,
        }
    }
}

impl<T: Vertex, V: AsIterator<Item = T> + FromIterator<T>> FromIterator<T> for Polygon<V, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(V::from_iter(iter))
    }
}

impl<T: Vertex + PartialEq, U: AsIterator<Item = T> + ?Sized, V: AsIterator<Item = T> + ?Sized>
    PartialEq<Polygon<U, T>> for Polygon<V, T>
{
    fn eq(&self, other: &Polygon<U, T>) -> bool {
        self.vertices().eq(other.vertices())
    }
}

impl<T: Vertex, V: AsIterator<Item = T> + ?Sized> Polygon<V, T> {
    /// Get an iterator over the vertices of the polygon.
    pub fn vertices(&self) -> Copied<V::RefIter<'_>> {
        self.vertices.iter().copied()
    }

    /// Check if the polygon has no vertices.
    pub fn is_empty(&self) -> bool {
        self.vertices().next().is_none()
    }

    fn vertices_window<const N: usize>(&self) -> impl Iterator<Item = [T; N]> {
        self.vertices()
            .chain(self.vertices()) // If window size is greater that number of vertices then iterator is empty
            .scan([None; N], |w, v| {
                w.rotate_left(1);
                w[N - 1] = Some(v);
                Some(*w)
            })
            .skip(N - 1) // Fill window
            .zip(self.vertices()) // Take the same number of items as number of vertices
            .map(|(w, _)| w.map(|v| v.unwrap()))
    }

    /// Get an iterator over the edges of the polygon.
    ///
    /// The edges are formed by connecting consecutive vertices,
    /// including an edge from the last vertex back to the first.
    pub fn edges(&self) -> impl Iterator<Item = T::Edge> {
        self.vertices_window()
            .map(|[a, b]| T::Edge::from_vertices(&a, &b))
    }
}

impl<T: Vertex, V: AsIterator<Item = T> + ?Sized> Polygon<V, T>
where
    for<'a> V::RefIter<'a>: ExactSizeIterator,
{
    /// Get the number of vertices in the polygon.
    pub fn len(&self) -> usize {
        self.vertices().len()
    }
}

impl<T: Vertex, V: AsIterator<Item = T> + Debug + ?Sized> Debug for Polygon<V, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Polygon {{ vertices: {:?} }}", &self.vertices)
    }
}
