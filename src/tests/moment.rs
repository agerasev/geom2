use crate::{Closed, Disk, EPS, Integrable, Moment};
use approx::assert_abs_diff_eq;
use either::Either;
use glam::Vec2;

const TEST_EPS: f32 = 1e-6;

#[test]
fn moment_merge() {
    let moment1 = Moment {
        area: 10.0,
        centroid: Vec2::new(1.0, 2.0),
    };
    let moment2 = Moment {
        area: 20.0,
        centroid: Vec2::new(3.0, 4.0),
    };

    let merged = moment1.merge(moment2);

    // Area should sum
    assert_abs_diff_eq!(merged.area, 30.0, epsilon = TEST_EPS);

    // Centroid should be weighted average
    let expected_centroid =
        (moment1.centroid * moment1.area + moment2.centroid * moment2.area) / 30.0;
    assert_abs_diff_eq!(merged.centroid, expected_centroid, epsilon = TEST_EPS);

    // Merging with zero area should return default
    let zero_moment = Moment {
        area: 0.0,
        centroid: Vec2::new(100.0, 200.0),
    };
    let merged_with_zero = moment1.merge(zero_moment);
    assert_abs_diff_eq!(merged_with_zero.area, moment1.area, epsilon = TEST_EPS);
    assert_abs_diff_eq!(
        merged_with_zero.centroid,
        moment1.centroid,
        epsilon = TEST_EPS
    );

    // Merging two zero areas should return default
    let zero_moment2 = Moment {
        area: 0.0,
        centroid: Vec2::new(300.0, 400.0),
    };
    let zero_merged = zero_moment.merge(zero_moment2);
    assert_abs_diff_eq!(zero_merged.area, 0.0, epsilon = TEST_EPS);
    assert_abs_diff_eq!(zero_merged.centroid, Vec2::ZERO, epsilon = TEST_EPS);
}

#[test]
fn moment_approx_eq() {
    let moment1 = Moment {
        area: 1.0,
        centroid: Vec2::new(2.0, 3.0),
    };
    let moment2 = Moment {
        area: 1.0 + EPS / 2.0,
        centroid: Vec2::new(2.0 + EPS / 2.0, 3.0 + EPS / 2.0),
    };

    // Should be approximately equal within EPS (difference is 0.5 * EPS)
    assert_abs_diff_eq!(moment1, moment2, epsilon = EPS);

    // Not equal with larger difference (10 * EPS > EPS)
    let moment3 = Moment {
        area: 1.0 + 10.0 * EPS,
        centroid: Vec2::new(2.0 + 10.0 * EPS, 3.0 + 10.0 * EPS),
    };
    assert!(!approx::abs_diff_eq!(&moment1, &moment3, epsilon = EPS));
}

#[test]
fn option_closed_integrable() {
    let disk = Disk::new(Vec2::new(1.0, 2.0), 3.0);

    // Some disk
    let some_disk: Option<Disk> = Some(disk);

    // Test Closed trait
    assert!(some_disk.contains(Vec2::new(1.0, 2.0))); // center
    assert!(!some_disk.contains(Vec2::new(10.0, 10.0))); // outside

    // Test Integrable trait
    let moment = some_disk.moment();
    assert_abs_diff_eq!(moment.area, disk.area(), epsilon = TEST_EPS);
    assert_abs_diff_eq!(moment.centroid, disk.centroid(), epsilon = TEST_EPS);

    // None
    let none_disk: Option<Disk> = None;

    // Test Closed trait - None should never contain anything
    assert!(!none_disk.contains(Vec2::new(0.0, 0.0)));
    assert!(!none_disk.contains(Vec2::new(1.0, 1.0)));

    // Test Integrable trait - None should have zero moment
    let none_moment = none_disk.moment();
    assert_abs_diff_eq!(none_moment.area, 0.0, epsilon = TEST_EPS);
    assert_abs_diff_eq!(none_moment.centroid, Vec2::ZERO, epsilon = TEST_EPS);
}

#[test]
fn either_closed_integrable() {
    let disk1 = Disk::new(Vec2::new(0.0, 0.0), 1.0);
    let disk2 = Disk::new(Vec2::new(5.0, 5.0), 2.0);

    // Left variant
    let left: Either<Disk, Disk> = Either::Left(disk1);

    // Test Closed trait - should behave like left disk
    assert!(left.contains(Vec2::new(0.0, 0.0)));
    assert!(!left.contains(Vec2::new(5.0, 5.0)));

    // Test Integrable trait
    let left_moment = left.moment();
    assert_abs_diff_eq!(left_moment.area, disk1.area(), epsilon = TEST_EPS);
    assert_abs_diff_eq!(left_moment.centroid, disk1.centroid(), epsilon = TEST_EPS);

    // Right variant
    let right: Either<Disk, Disk> = Either::Right(disk2);

    // Test Closed trait - should behave like right disk
    assert!(right.contains(Vec2::new(5.0, 5.0)));
    assert!(!right.contains(Vec2::new(0.0, 0.0)));

    // Test Integrable trait
    let right_moment = right.moment();
    assert_abs_diff_eq!(right_moment.area, disk2.area(), epsilon = TEST_EPS);
    assert_abs_diff_eq!(right_moment.centroid, disk2.centroid(), epsilon = TEST_EPS);

    // Test with different types (Disk and Option<Disk>)
    let disk = Disk::new(Vec2::new(1.0, 1.0), 2.0);
    let none_disk: Option<Disk> = None;

    let left_mixed: Either<Disk, Option<Disk>> = Either::Left(disk);
    assert!(left_mixed.contains(Vec2::new(1.0, 1.0)));
    assert_abs_diff_eq!(left_mixed.area(), disk.area(), epsilon = TEST_EPS);

    let right_mixed: Either<Disk, Option<Disk>> = Either::Right(none_disk);
    assert!(!right_mixed.contains(Vec2::new(0.0, 0.0)));
    assert_abs_diff_eq!(right_mixed.area(), 0.0, epsilon = TEST_EPS);
}
