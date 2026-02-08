pub mod line;

pub trait Edge: Copy {
    type Vertex: Vertex<Edge = Self>;
    fn from_vertices(a: &Self::Vertex, b: &Self::Vertex) -> Self;
}
pub trait Vertex: Copy {
    type Edge: Edge<Vertex = Self>;
}

pub trait AsIterator {
    type Item;
    type RefIter<'a>: Iterator<Item = &'a Self::Item>
    where
        Self: 'a;
    fn iter(&self) -> Self::RefIter<'_>;
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

#[derive(Clone, Copy, Debug)]
pub struct Polygon<V: AsIterator + ?Sized> {
    pub vertices: V,
}

impl<V: AsIterator> Polygon<V> {
    pub fn new(vertices: V) -> Self {
        Self { vertices }
    }
}

impl<V: AsIterator + FromIterator<V::Item>> FromIterator<V::Item> for Polygon<V> {
    fn from_iter<T: IntoIterator<Item = V::Item>>(iter: T) -> Self {
        Self::new(V::from_iter(iter))
    }
}

impl<T: PartialEq, U: AsIterator<Item = T> + ?Sized, V: AsIterator<Item = T> + ?Sized>
    PartialEq<Polygon<U>> for Polygon<V>
{
    fn eq(&self, other: &Polygon<U>) -> bool {
        self.vertices().eq(other.vertices())
    }
}

impl<V: AsIterator + ?Sized> Polygon<V> {
    pub fn vertices(&self) -> V::RefIter<'_> {
        self.vertices.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.vertices().next().is_none()
    }

    fn vertices_window<const N: usize>(&self) -> impl Iterator<Item = [&V::Item; N]> {
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
}

impl<V: AsIterator + ?Sized> Polygon<V>
where
    for<'a> V::RefIter<'a>: ExactSizeIterator,
{
    pub fn len(&self) -> usize {
        self.vertices.iter().len()
    }
}

impl<T: Vertex, V: AsIterator<Item = T> + ?Sized> Polygon<V> {
    pub fn edges(&self) -> impl Iterator<Item = T::Edge> {
        self.vertices_window()
            .map(|[a, b]| T::Edge::from_vertices(a, b))
    }
}
