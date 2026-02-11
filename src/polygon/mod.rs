pub mod circle;
pub mod line;

use crate::AsIterator;
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
pub struct Polygon<V: AsIterator<Item = T> + ?Sized, T: Vertex = Vec2> {
    _ghost: PhantomData<T>,
    pub vertices: V,
}

impl<T: Vertex, V: AsIterator<Item = T>> Polygon<V, T> {
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
    pub fn vertices(&self) -> V::RefIter<'_> {
        self.vertices.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices().next().is_none()
    }

    fn vertices_window<const N: usize>(&self) -> impl Iterator<Item = [&T; N]> {
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

    pub fn edges(&self) -> impl Iterator<Item = T::Edge> {
        self.vertices_window()
            .map(|[a, b]| T::Edge::from_vertices(a, b))
    }
}

impl<T: Vertex, V: AsIterator<Item = T> + ?Sized> Polygon<V, T>
where
    for<'a> V::RefIter<'a>: ExactSizeIterator,
{
    pub fn len(&self) -> usize {
        self.vertices().len()
    }
}
