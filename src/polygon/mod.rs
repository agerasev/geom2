pub mod circle;
pub mod line;

use core::marker::PhantomData;
use glam::Vec2;

pub trait Edge: Copy {
    type Vertex: Vertex<Edge = Self>;
    fn from_vertices(a: &Self::Vertex, b: &Self::Vertex) -> Self;
}
pub trait Vertex: Copy {
    type Edge: Edge<Vertex = Self>;
}

#[derive(Clone, Copy, Debug)]
pub struct Polygon<V: AsRef<[T]> + ?Sized, T: Vertex = Vec2> {
    _ghost: PhantomData<T>,
    pub vertices: V,
}

impl<T: Vertex, V: AsRef<[T]>> Polygon<V, T> {
    pub fn new(vertices: V) -> Self {
        Self {
            vertices,
            _ghost: PhantomData,
        }
    }
}

impl<T: Vertex, V: AsRef<[T]> + FromIterator<T>> FromIterator<T> for Polygon<V, T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self::new(V::from_iter(iter))
    }
}

impl<T: Vertex + PartialEq, U: AsRef<[T]> + ?Sized, V: AsRef<[T]> + ?Sized> PartialEq<Polygon<U, T>>
    for Polygon<V, T>
{
    fn eq(&self, other: &Polygon<U, T>) -> bool {
        self.vertices().eq(other.vertices())
    }
}

impl<T: Vertex, V: AsRef<[T]> + ?Sized> Polygon<V, T> {
    pub fn vertices(&self) -> &[T] {
        self.vertices.as_ref()
    }

    pub fn len(&self) -> usize {
        self.vertices().len()
    }
    pub fn is_empty(&self) -> bool {
        self.vertices().is_empty()
    }

    fn vertices_window<const N: usize>(&self) -> impl Iterator<Item = [&T; N]> {
        self.vertices()
            .iter()
            .chain(self.vertices().iter()) // If window size is greater that number of vertices then iterator is empty
            .scan([None; N], |w, v| {
                w.rotate_left(1);
                w[N - 1] = Some(v);
                Some(*w)
            })
            .skip(N - 1) // Fill window
            .take(self.len()) // Take the same number of items as number of vertices
            .map(|w| w.map(|v| v.unwrap()))
    }

    pub fn edges(&self) -> impl Iterator<Item = T::Edge> {
        self.vertices_window()
            .map(|[a, b]| T::Edge::from_vertices(a, b))
    }
}
