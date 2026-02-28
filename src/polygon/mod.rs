pub mod circle;
pub mod line;

use crate::{CopyIterator, EPS, Edge, Integrable, Polygon, Vertex};
use core::{
    fmt::{self, Debug, Formatter},
    marker::PhantomData,
};
use glam::Vec2;

/// A polygon defined by a sequence of vertices.
///
/// The polygon can have any vertex type `T` that implements [`Vertex`],
/// and the vertices can be stored in any container `V` that implements [`CopyIterator`].
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
pub struct GenericPolygon<V: CopyIterator<Item = T> + ?Sized, T: Vertex> {
    _ghost: PhantomData<T>,
    /// The vertices of the polygon.
    pub vertices: V,
}

impl<T: Vertex, V: CopyIterator<Item = T>> GenericPolygon<V, T> {
    /// Create a new polygon from a sequence of vertices.
    pub fn new(vertices: V) -> Self {
        Self {
            vertices,
            _ghost: PhantomData,
        }
    }
}

impl<T: Vertex, V: CopyIterator<Item = T> + FromIterator<T>> FromIterator<T>
    for GenericPolygon<V, T>
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(V::from_iter(iter))
    }
}

impl<T: Vertex + PartialEq, U: CopyIterator<Item = T> + ?Sized, V: CopyIterator<Item = T> + ?Sized>
    PartialEq<GenericPolygon<U, T>> for GenericPolygon<V, T>
{
    fn eq(&self, other: &GenericPolygon<U, T>) -> bool {
        self.vertices().eq(other.vertices())
    }
}

impl<T: Vertex, V: CopyIterator<Item = T> + ?Sized> GenericPolygon<V, T> {
    /// Get an iterator over the vertices of the polygon.
    pub fn vertices(&self) -> V::CopyIter<'_> {
        self.vertices.iter_copied()
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

impl<T: Vertex, V: CopyIterator<Item = T> + ?Sized> GenericPolygon<V, T>
where
    for<'a> V::CopyIter<'a>: ExactSizeIterator,
{
    /// Get the number of vertices in the polygon.
    pub fn len(&self) -> usize {
        self.vertices().len()
    }
}

/// A polygon that can be converted to a polygonal frame.
///
/// This trait is implemented by polygons that can produce a "frame" representation
/// consisting of straight line segments (as opposed to curved edges).
/// The frame is useful for operations that require a simple polygon representation.
pub trait FramedPolygon {
    /// Convert the polygon to its polygonal frame.
    ///
    /// Returns a polygon where all curved edges (e.g., arcs) are approximated
    /// by their chords (straight line segments between endpoints).
    fn frame(&self) -> Polygon<impl CopyIterator<Item = Vec2> + '_>;

    /// Determine the orientation of the polygon.
    ///
    /// Returns:
    /// - `1` if the polygon is counterclockwise (CCW)
    /// - `-1` if the polygon is clockwise (CW)
    /// - `0` if the polygon is degenerate (area ≈ 0)
    fn orientation(&self) -> i32 {
        let area = self.frame().area();
        if area.abs() < EPS {
            0
        } else if area > 0.0 {
            1
        } else {
            -1
        }
    }
}

impl<T: Vertex, V: CopyIterator<Item = T> + Debug + ?Sized> Debug for GenericPolygon<V, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Polygon {{ vertices: {:?} }}", &self.vertices)
    }
}
