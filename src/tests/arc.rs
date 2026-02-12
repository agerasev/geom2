use crate::{Arc, Bounded, DiskSegment, EPS, Integrate, Moment};
use approx::assert_abs_diff_eq;
use core::f32::consts::PI;
use glam::Vec2;

const R: f32 = 1.234;

#[test]
fn empty_segment() {
    let Moment { area, centroid } = DiskSegment(Arc {
        points: (Vec2::new(EPS, 0.0), Vec2::new(-EPS, 0.0)),
        sagitta: 0.0,
    })
    .moment();

    assert_abs_diff_eq!(area, 0.0, epsilon = EPS);
    assert_abs_diff_eq!(centroid, Vec2::ZERO, epsilon = EPS);
}

#[test]
fn full_segment() {
    let Moment { area, centroid } = DiskSegment(Arc {
        points: (Vec2::new(EPS, 0.0), Vec2::new(-EPS, 0.0)),
        sagitta: 2.0 * R,
    })
    .moment();

    assert_abs_diff_eq!(area, PI * R.powi(2), epsilon = EPS);
    assert_abs_diff_eq!(centroid, Vec2::new(0.0, R), epsilon = EPS);
}

#[test]
fn half_segment() {
    assert_eq!(
        DiskSegment(Arc {
            points: (Vec2::new(R, 0.0), Vec2::new(-R, 0.0)),
            sagitta: R,
        })
        .area(),
        PI * R.powi(2) / 2.0
    );
}

#[test]
fn segment_contains() {
    let segment = DiskSegment(Arc {
        points: (Vec2::new(4.0, 1.0), Vec2::new(1.0, 1.0)),
        sagitta: 4.0,
    });

    assert!(!segment.contains(Vec2::new(2.5, 5.01)));
    assert!(segment.contains(Vec2::new(2.5, 4.99)));

    assert!(segment.contains(Vec2::new(2.5, 1.01)));
    assert!(!segment.contains(Vec2::new(2.5, 0.99)));
}

#[test]
fn numerical_segment() {
    let f = |x: f64| 2.0 * (1.0 - (1.0 - x).powi(2)).sqrt();

    let mut x: f64 = 0.0;
    let dx: f64 = 1e-6;

    let (mut area, mut moment) = (0.0, 0.0);

    let check_step = 1e-2;
    let mut last_check = 0.0;
    while x < 2.0 {
        let d_area = 0.5 * (f(x) + f(x + dx)) * dx;
        area += d_area;
        moment += d_area * (x + 0.5 * dx);
        if x >= last_check + check_step {
            last_check = x;
            let y = (1.0 - (1.0 - x).powi(2)).sqrt();
            let ref_segment = DiskSegment(Arc {
                points: (
                    Vec2::new(x as f32, y as f32),
                    Vec2::new(x as f32, -y as f32),
                ),
                sagitta: x as f32,
            });
            assert_abs_diff_eq!(ref_segment.area(), area as f32, epsilon = 1e-4);
            assert_abs_diff_eq!(
                ref_segment.centroid(),
                Vec2::new((moment / area) as f32, 0.0),
                epsilon = 1e-4
            );
        }
        x += dx;
    }
}
