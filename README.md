# geom2

A Rust library for 2D geometry primitives and operations with `no_std` support.

[![Crates.io](https://img.shields.io/crates/v/geom2)](https://crates.io/crates/geom2)
[![Documentation](https://docs.rs/geom2/badge.svg)](https://docs.rs/geom2)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

`geom2` provides a comprehensive set of 2D geometric primitives and operations for computational geometry applications. The library is designed with performance and correctness in mind, featuring robust handling of edge cases and floating-point precision issues.

### Key Features

- **`no_std` compatible** - Works in embedded and constrained environments
- **Comprehensive primitives** - Lines, circles, arcs, polygons, half-planes
- **Geometric operations** - Intersection, containment, area calculation, winding numbers
- **Robust floating-point handling** - EPS-based tolerance for numerical stability
- **Generic design** - Flexible vertex and edge types with iterator-based APIs
- **Approximation support** - Optional `approx` feature for approximate equality comparisons

## Installation

Add `geom2` to your `Cargo.toml`:

```toml
[dependencies]
geom2 = "0.1"
```

For approximate equality comparisons, enable the `approx` feature:

```toml
[dependencies]
geom2 = { version = "0.1", features = ["approx"] }
```

## Primitives

### Basic Shapes

- **`Line`** - Infinite line defined by two points
- **`LineSegment`** - Finite line segment between two points
- **`Circle`** - Circle defined by center and radius
- **`Disk`** - Filled circle (circle with interior)
- **`Arc`** - Circular arc segment
- **`HalfPlane`** - Half-plane defined by a boundary line
- **`Polygon`** - Polygon with generic vertex storage

### Composite Types

- **`ArcPolygon`** - Polygon with circular arc edges
- **`DiskSegment`** - Segment of a disk (intersection of disk and half-plane)

## Core Traits

The library is built around several key traits that define geometric behavior:

### `Closed`
Shapes that have an oriented boundary, supporting winding number calculations:
```rust
pub trait Closed {
    fn winding_number_2(&self, point: Vec2) -> i32;
    fn contains(&self, point: Vec2) -> bool;
}
```

### `Integrable`
Shapes that have computable geometric moments (area, centroid):
```rust
pub trait Integrable {
    fn moment(&self) -> Moment;
    fn area(&self) -> f32;
    fn centroid(&self) -> Vec2;
}
```

### `Intersect`
Shapes that can compute intersections with other shapes:
```rust
pub trait Intersect<T: Intersect<Self, Output = Self::Output> + ?Sized> {
    type Output: Sized;
    fn intersect(&self, other: &T) -> Option<Self::Output>;
}
```

### `Edge` and `Vertex`
Traits for defining polygon edges and vertices, allowing for flexible polygon representations.

## Usage Examples

### Basic Shape Creation

```rust
use geom2::{Line, LineSegment, Circle, Disk, Polygon};
use glam::Vec2;

// Create a line through points (0, 0) and (1, 1)
let line = Line(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));

// Create a line segment
let segment = LineSegment(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));

// Create a circle at (2, 3) with radius 5
let circle = Circle {
    center: Vec2::new(2.0, 3.0),
    radius: 5.0,
};

// Create a filled disk
let disk = Disk::new(Vec2::new(2.0, 3.0), 5.0);

// Create a triangle polygon
let triangle = Polygon::new([
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(0.5, 1.0),
]);
```

### Geometric Operations

```rust
use geom2::{Line, LineSegment, Circle, Intersect};
use glam::Vec2;

// Line-line intersection
let line1 = Line(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0));
let line2 = Line(Vec2::new(0.0, 1.0), Vec2::new(1.0, 0.0));
let intersection = line1.intersect(&line2); // Some(Vec2::new(0.5, 0.5))

// Line-circle intersection
let circle = Circle {
    center: Vec2::new(0.0, 0.0),
    radius: 1.0,
};
let intersections = circle.intersect(&line1); // Some([Vec2; 2])

// Check if point is inside shape
use geom2::Closed;
let disk = Disk::new(Vec2::new(0.0, 0.0), 1.0);
let inside = disk.contains(Vec2::new(0.5, 0.5)); // true

// Compute area and centroid
use geom2::Integrable;
let area = disk.area(); // π
let centroid = disk.centroid(); // Vec2::new(0.0, 0.0)
```

### Polygon Operations

```rust
use geom2::{Polygon, Closed, Integrable};
use glam::Vec2;

// Create a square polygon
let square = Polygon::new([
    Vec2::new(0.0, 0.0),
    Vec2::new(1.0, 0.0),
    Vec2::new(1.0, 1.0),
    Vec2::new(0.0, 1.0),
]);

// Check orientation (should be CCW)
let orientation = square.orientation(); // 1

// Iterate over edges
for edge in square.edges() {
    // Process each edge
}

// Compute area
let area = square.area(); // 1.0

// Check point containment
let inside = square.contains(Vec2::new(0.5, 0.5)); // true
```

### Advanced Operations with `intersect_to`

The `IntersectTo` trait provides flexible intersection operations where you can specify the output type.
See `examples/intersect_to.rs` file for `intersect_to` example.

## Numerical Precision

The library uses a global `EPS` constant (`1e-8`) for floating-point comparisons. All geometric operations are designed to handle edge cases and numerical instability within this tolerance.

## Features

- **`approx`** - Enables approximate equality comparisons using the `approx` crate. When enabled, geometric types implement `approx::AbsDiffEq` and `approx::RelativeEq`.

## Design Philosophy

1. **Correctness First** - Robust handling of degenerate cases and floating-point precision
2. **Performance** - Efficient algorithms with minimal allocations
3. **Flexibility** - Generic types that work with various storage backends
4. **`no_std` Compatibility** - Suitable for embedded and constrained environments

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.