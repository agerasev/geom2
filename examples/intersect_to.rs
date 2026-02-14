use geom2::{ArcPolygon, Disk, HalfPlane, Intersect, IntersectTo, Line, Polygon};
use glam::Vec2;

fn main() {
    // Create two overlapping polygons
    let square = Polygon::new([
        Vec2::new(0.0, 0.0),
        Vec2::new(2.0, 0.0),
        Vec2::new(2.0, 2.0),
        Vec2::new(0.0, 2.0),
    ]);

    let triangle = Polygon::new([
        Vec2::new(1.0, 1.0),
        Vec2::new(3.0, 1.0),
        Vec2::new(2.0, 3.0),
    ]);

    // Polygon-polygon intersection (returns another polygon)
    let intersection: Option<Polygon<Vec<_>>> = square.intersect_to(&triangle);
    println!("Polygon-polygon intersection: {:?}", intersection);

    // Polygon-halfplane intersection (clipping)
    let halfplane = HalfPlane::from_edge(Line(Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0)));
    let clipped: Option<Polygon<Vec<_>>> = square.intersect_to(&halfplane);
    println!("Polygon-halfplane intersection: {:?}", clipped);

    // Polygon-disk intersection (returns arc polygon)
    let disk = Disk::new(Vec2::new(1.0, 1.0), 1.5);
    let arc_polygon: Option<ArcPolygon<Vec<_>>> = square.intersect_to(&disk);
    println!("Polygon-disk intersection: {:?}", arc_polygon);

    // The regular `intersect` method can also be used for simpler cases:
    let intersection = disk.intersect(&halfplane);
    println!("Disk-halfplane intersection: {:?}", intersection);
}
