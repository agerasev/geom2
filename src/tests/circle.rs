extern crate std;

use crate::{Closed, Disk, HalfPlane, Integrable, Intersect};
use approx::assert_abs_diff_eq;
use either::Either;
use glam::Vec2;
use std::vec::Vec;

const TEST_EPS: f32 = 1e-6;

#[test]
fn contains() {
    let disk = Disk::new(Vec2::new(0.0, 0.0), 1.0);

    assert!(disk.contains(disk.center));

    // Inside points
    assert!(disk.contains(Vec2::new(0.5, 0.0)));
    assert!(disk.contains(Vec2::new(0.0, 0.5)));
    assert!(disk.contains(Vec2::new(0.3, 0.4)));

    // Outside points
    assert!(!disk.contains(Vec2::new(1.5, 0.0)));
    assert!(!disk.contains(Vec2::new(0.0, 1.5)));
    assert!(!disk.contains(Vec2::new(0.9, 0.9)));
}

#[test]
fn intersect_half_plane_outside() {
    // Disk completely outside half-plane
    let disk = Disk::new(Vec2::new(0.0, 0.0), 1.0);
    let half_plane = HalfPlane::from_normal(Vec2::new(2.0, 0.0), Vec2::new(1.0, 0.0));
    // Disk center at (0,0), half-plane boundary at x=2, normal points right (positive x)
    // Disk radius 1, so furthest point is at x=1, which is still left of boundary at x=2
    // Since normal points from inside to outside, and disk is completely on the "outside" side
    // (distance from center to plane = -2, apothem = 2 > radius 1)
    assert!(disk.intersect(&half_plane).is_none());
}

#[test]
fn intersect_half_plane_inside() {
    // Disk completely inside half-plane
    let disk = Disk::new(Vec2::new(0.0, 0.0), 1.0);
    let half_plane = HalfPlane::from_normal(Vec2::new(-2.0, 0.0), Vec2::new(1.0, 0.0));
    // Disk center at (0,0), half-plane boundary at x=-2, normal points right (positive x)
    // Disk is completely on the "inside" side (distance from center to plane = 2, apothem = -2 < -radius)
    let result = disk.intersect(&half_plane);
    assert!(result.is_some());
    match result.unwrap() {
        Either::Right(returned_disk) => {
            assert_eq!(returned_disk, disk);
        }
        Either::Left(_) => panic!("Expected disk to be completely inside half-plane"),
    }
}

#[test]
fn intersect_half_plane_intersecting() {
    // Disk intersecting half-plane
    let disk = Disk::new(Vec2::new(0.0, 0.0), 1.0);
    let half_plane = HalfPlane::from_normal(Vec2::new(0.5, 0.0), Vec2::new(1.0, 0.0));
    // Disk center at (0,0), half-plane boundary at x=0.5, normal points right
    // Disk intersects the half-plane (distance from center to plane = -0.5, apothem = 0.5 < radius 1)
    let result = disk.intersect(&half_plane);
    assert!(result.is_some());
    match result.unwrap() {
        Either::Left(disk_segment) => {
            // Verify the disk segment properties
            let arc = disk_segment.0;
            // Chord endpoints should be symmetric about the x-axis
            assert_abs_diff_eq!(arc.points.0.y, -arc.points.1.y, epsilon = TEST_EPS);
            assert_abs_diff_eq!(arc.points.0.x, arc.points.1.x, epsilon = TEST_EPS);
            // Chord midpoint should be at (0.5, 0) since apothem = 0.5
            let chord_mid = 0.5 * (arc.points.0 + arc.points.1);
            assert_abs_diff_eq!(chord_mid.x, 0.5, epsilon = TEST_EPS);
            assert_abs_diff_eq!(chord_mid.y, 0.0, epsilon = TEST_EPS);
            // Sagitta should be radius - apothem = 1.0 - 0.5 = 0.5
            assert_abs_diff_eq!(arc.sagitta, 0.5, epsilon = TEST_EPS);
            // Chord length should be 2*h where h = sqrt(radius^2 - apothem^2) = sqrt(1 - 0.25) = sqrt(0.75)
            let expected_h = (1.0f32 - 0.5f32.powi(2)).sqrt();
            let chord_vec = arc.points.1 - arc.points.0;
            assert_abs_diff_eq!(chord_vec.length() / 2.0, expected_h, epsilon = TEST_EPS);
        }
        Either::Right(_) => panic!("Expected disk to intersect half-plane"),
    }
}

#[test]
fn intersect_half_plane_tangent() {
    // Disk tangent to half-plane (edge case)
    let disk = Disk::new(Vec2::new(0.0, 0.0), 1.0);
    let half_plane = HalfPlane::from_normal(Vec2::new(1.0, 0.0), Vec2::new(1.0, 0.0));
    // Disk center at (0,0), half-plane boundary at x=1, normal points right
    // Disk is tangent to half-plane (distance from center to plane = -1, apothem = 1 = radius)
    let result = disk.intersect(&half_plane);
    assert!(result.is_some());
    match result.unwrap() {
        Either::Left(disk_segment) => {
            // Tangent case should produce a disk segment with zero area
            let arc = disk_segment.0;
            // Chord endpoints should be the same point (or very close)
            assert_abs_diff_eq!(arc.points.0, arc.points.1, epsilon = TEST_EPS);
            // Sagitta should be radius - apothem = 1.0 - 1.0 = 0.0
            assert_abs_diff_eq!(arc.sagitta, 0.0, epsilon = TEST_EPS);
        }
        Either::Right(_) => panic!("Expected tangent intersection to produce disk segment"),
    }
}

#[test]
fn intersect_half_plane_negative_apothem() {
    // Disk intersecting half-plane with negative apothem
    // Center is inside half-plane, but disk extends outside
    let disk = Disk::new(Vec2::new(0.0, 0.0), 2.0);
    let half_plane = HalfPlane::from_normal(Vec2::new(-1.0, 0.0), Vec2::new(1.0, 0.0));
    // Disk center at (0,0), half-plane boundary at x=-1, normal points right
    // Distance from center to plane = 0.dot((1,0)) - (-1) = 1
    // Since point is outside (distance positive), apothem = -distance = -1
    // apothem = -1, radius = 2, so |apothem| < radius → intersection
    // apothem is negative (center inside half-plane)
    let result = disk.intersect(&half_plane);
    assert!(result.is_some());
    match result.unwrap() {
        Either::Left(disk_segment) => {
            let arc = disk_segment.0;
            // Verify the disk segment properties
            // Chord endpoints should be symmetric about the x-axis
            assert_abs_diff_eq!(arc.points.0.y, -arc.points.1.y, epsilon = TEST_EPS);
            assert_abs_diff_eq!(arc.points.0.x, arc.points.1.x, epsilon = TEST_EPS);
            // Chord midpoint should be at (-1, 0) since apothem = -1
            // (center at (0,0), move -1 along normal (1,0) to get to chord midpoint)
            let chord_mid = 0.5 * (arc.points.0 + arc.points.1);
            assert_abs_diff_eq!(chord_mid.x, -1.0, epsilon = TEST_EPS);
            assert_abs_diff_eq!(chord_mid.y, 0.0, epsilon = TEST_EPS);
            // Sagitta should be radius - apothem = 2.0 - (-1.0) = 3.0
            // (since apothem is negative, sagitta > radius)
            assert_abs_diff_eq!(arc.sagitta, 3.0, epsilon = TEST_EPS);
            // Chord length should be 2*h where h = sqrt(radius^2 - apothem^2) = sqrt(4 - 1) = sqrt(3)
            let expected_h = (4.0f32 - 1.0f32).sqrt(); // radius^2 - apothem^2 = 4 - 1 = 3
            let chord_vec = arc.points.1 - arc.points.0;
            assert_abs_diff_eq!(chord_vec.length() / 2.0, expected_h, epsilon = TEST_EPS);
        }
        Either::Right(_) => panic!("Expected disk to intersect half-plane"),
    }
}

#[test]
fn intersect_disk_no_intersection() {
    // Two disks that don't intersect
    let disk1 = Disk::new(Vec2::new(0.0, 0.0), 1.0);
    let disk2 = Disk::new(Vec2::new(3.0, 0.0), 1.0);
    // Distance between centers = 3, sum of radii = 2, so no intersection
    assert!(disk1.intersect(&disk2).is_none());
    // Test commutative property
    assert!(disk2.intersect(&disk1).is_none());
}

#[test]
fn intersect_disk_inside() {
    // One disk completely inside another
    let disk1 = Disk::new(Vec2::new(0.0, 0.0), 2.0); // Larger disk
    let disk2 = Disk::new(Vec2::new(0.5, 0.0), 1.0); // Smaller disk inside
    // disk2 is completely inside disk1
    let result = disk1.intersect(&disk2);
    assert!(result.is_some());
    match result.unwrap() {
        Either::Right(smaller_disk) => {
            // Should return the smaller disk (disk2)
            assert_eq!(smaller_disk, disk2);
        }
        Either::Left(_) => panic!("Expected smaller disk to be completely inside larger disk"),
    }

    // Test commutative property - should also return the smaller disk
    let result = disk2.intersect(&disk1);
    assert!(result.is_some());
    match result.unwrap() {
        Either::Right(smaller_disk) => {
            assert_eq!(smaller_disk, disk2);
        }
        Either::Left(_) => panic!("Expected commutative property to hold"),
    }
}

#[test]
fn intersect_disk_intersecting() {
    // Two disks that intersect at two points
    let disk1 = Disk::new(Vec2::new(0.0, 0.0), 2.0);
    let disk2 = Disk::new(Vec2::new(3.0, 0.0), 2.0);
    // Distance between centers = 3, sum of radii = 4, difference of radii = 0
    // So they intersect at two points
    let result = disk1.intersect(&disk2);
    assert!(result.is_some());
    match result.unwrap() {
        Either::Left(polygon) => {
            // Should be a polygon with 2 arc vertices
            let vertices: Vec<_> = polygon.vertices().collect();
            assert_eq!(vertices.len(), 2);

            // Check that vertices are ArcVertex type
            let v0 = vertices[0];
            let v1 = vertices[1];

            // The intersection points should be symmetric about the x-axis
            // since circles are centered at (0,0) and (3,0) with equal radii
            assert_abs_diff_eq!(v0.point.y, -v1.point.y, epsilon = TEST_EPS);
            assert_abs_diff_eq!(v0.point.x, v1.point.x, epsilon = TEST_EPS);

            // The x-coordinate should be at the midpoint between centers
            // which is at x = 1.5 (halfway between 0 and 3)
            assert_abs_diff_eq!(v0.point.x, 1.5, epsilon = TEST_EPS);

            // The y-coordinate should be sqrt(radius^2 - (distance/2)^2)
            // = sqrt(4 - 2.25) = sqrt(1.75) ≈ 1.3228756
            let expected_y = (4.0f32 - 1.5f32.powi(2)).sqrt();
            assert_abs_diff_eq!(v0.point.y.abs(), expected_y, epsilon = TEST_EPS);

            // Sagittas should be positive (arcs bulge outward from chord)
            // For equal radii, sagittas should be equal
            assert_abs_diff_eq!(v0.sagitta, v1.sagitta, epsilon = TEST_EPS);
            assert!(v0.sagitta > 0.0);
        }
        Either::Right(_) => panic!("Expected disks to intersect at two points"),
    }

    // Test commutative property
    let result = disk2.intersect(&disk1);
    assert!(result.is_some());
    match result.unwrap() {
        Either::Left(polygon) => {
            let vertices: Vec<_> = polygon.vertices().collect();
            assert_eq!(vertices.len(), 2);
        }
        Either::Right(_) => panic!("Expected commutative property to hold"),
    }
}

#[test]
fn intersect_disk_tangent() {
    // Two disks tangent to each other (touching at one point)
    let disk1 = Disk::new(Vec2::new(0.0, 0.0), 1.0);
    let disk2 = Disk::new(Vec2::new(2.0, 0.0), 1.0);
    // Distance between centers = 2, sum of radii = 2, so tangent
    let result = disk1.intersect(&disk2);
    assert!(result.is_some());
    match result.unwrap() {
        Either::Left(polygon) => {
            // Tangent case should produce a polygon with 2 vertices
            let vertices: Vec<_> = polygon.vertices().collect();
            assert_eq!(vertices.len(), 2);

            // The intersection point should be at (1, 0)
            let v0 = vertices[0];
            let v1 = vertices[1];
            // Both vertices should be at the same point (or very close)
            assert_abs_diff_eq!(v0.point, v1.point, epsilon = TEST_EPS);
            assert_abs_diff_eq!(v0.point.x, 1.0, epsilon = TEST_EPS);
            assert_abs_diff_eq!(v0.point.y, 0.0, epsilon = TEST_EPS);

            // Sagittas should be 0 for tangent case
            assert_abs_diff_eq!(v0.sagitta, 0.0, epsilon = TEST_EPS);
            assert_abs_diff_eq!(v1.sagitta, 0.0, epsilon = TEST_EPS);
        }
        Either::Right(_) => panic!("Expected tangent intersection to produce polygon"),
    }
}

#[test]
fn intersect_disk_concentric() {
    // Concentric disks (same center)
    let disk1 = Disk::new(Vec2::new(0.0, 0.0), 2.0);
    let disk2 = Disk::new(Vec2::new(0.0, 0.0), 1.0);
    // Distance between centers = 0, so smaller disk is inside larger
    let result = disk1.intersect(&disk2);
    assert!(result.is_some());
    match result.unwrap() {
        Either::Right(smaller_disk) => {
            // Should return the smaller disk (disk2)
            assert_eq!(smaller_disk, disk2);
        }
        Either::Left(_) => panic!("Expected smaller disk to be completely inside larger disk"),
    }
}

#[test]
fn intersect_disk_negative_apothem() {
    // Two disks intersecting with negative apothem for the first disk
    // This happens when the smaller disk's center is closer to the intersection chord
    // than the larger disk's center
    let disk1 = Disk::new(Vec2::new(0.0, 0.0), 1.0); // Smaller disk
    let disk2 = Disk::new(Vec2::new(1.5, 0.0), 2.0); // Larger disk
    // Distance between centers = 1.5
    // self_apothem = 0.5 * (distance + (r1^2 - r2^2) / distance)
    // = 0.5 * (1.5 + (1 - 4) / 1.5) = 0.5 * (1.5 + (-3)/1.5) = 0.5 * (1.5 - 2.0) = 0.5 * (-0.5) = -0.25
    // Negative apothem means the chord is on the opposite side of disk1's center
    // relative to disk2's center
    let result = disk1.intersect(&disk2);
    assert!(result.is_some());
    match result.unwrap() {
        Either::Left(polygon) => {
            // Should be a polygon with 2 arc vertices
            let vertices: Vec<_> = polygon.vertices().collect();
            assert_eq!(vertices.len(), 2);

            let v0 = vertices[0];
            let v1 = vertices[1];

            // Verify geometric properties
            // The intersection points should be symmetric about the x-axis
            assert_abs_diff_eq!(v0.point.y, -v1.point.y, epsilon = TEST_EPS);

            // Calculate expected values
            let distance: f32 = 1.5;
            let r1: f32 = 1.0;
            let r2: f32 = 2.0;
            let self_apothem: f32 = 0.5 * (distance + (r1.powi(2) - r2.powi(2)) / distance);
            // let other_apothem: f32 = distance - self_apothem;

            // Verify apothem is negative
            assert!(self_apothem < 0.0, "self_apothem should be negative");
            assert_abs_diff_eq!(self_apothem, -0.25, epsilon = TEST_EPS);

            // Chord midpoint should be at self.center + dir * self_apothem
            // dir = (1, 0) since disk2 is at (1.5, 0)
            // So midpoint = (0,0) + (-0.25, 0) = (-0.25, 0)
            let expected_midpoint = Vec2::new(-0.25, 0.0);
            let actual_midpoint = 0.5 * (v0.point + v1.point);
            assert_abs_diff_eq!(actual_midpoint, expected_midpoint, epsilon = TEST_EPS);

            // Half chord length h = sqrt(r1^2 - self_apothem^2) = sqrt(1 - 0.0625) = sqrt(0.9375)
            let expected_h = (r1.powi(2) - self_apothem.powi(2)).sqrt();
            let chord_vec = v1.point - v0.point;
            assert_abs_diff_eq!(chord_vec.length() / 2.0, expected_h, epsilon = TEST_EPS);

            // Sagittas
            // For disk1: sagitta = r1 - self_apothem = 1.0 - (-0.25) = 1.25
            // For disk2: sagitta = r2 - other_apothem = 2.0 - 1.75 = 0.25
            // (other_apothem = distance - self_apothem = 1.5 - (-0.25) = 1.75)
            // One sagitta should be larger than the other
            assert_abs_diff_eq!(v0.sagitta, 1.25, epsilon = TEST_EPS);
            assert_abs_diff_eq!(v1.sagitta, 0.25, epsilon = TEST_EPS);

            // Points should be at expected positions
            // v0.point = midpoint - dir.perp() * h = (-0.25, 0) - (0, 1) * h = (-0.25, -h)
            // v1.point = midpoint + dir.perp() * h = (-0.25, 0) + (0, 1) * h = (-0.25, h)
            assert_abs_diff_eq!(v0.point.x, -0.25, epsilon = TEST_EPS);
            assert_abs_diff_eq!(v1.point.x, -0.25, epsilon = TEST_EPS);
            assert_abs_diff_eq!(v0.point.y, -expected_h, epsilon = TEST_EPS);
            assert_abs_diff_eq!(v1.point.y, expected_h, epsilon = TEST_EPS);
        }
        Either::Right(_) => panic!("Expected disks to intersect at two points"),
    }

    // Test commutative property
    let result2 = disk2.intersect(&disk1);
    assert!(result2.is_some());
    match result2.unwrap() {
        Either::Left(polygon) => {
            let vertices: Vec<_> = polygon.vertices().collect();
            assert_eq!(vertices.len(), 2);
            // When swapping disks, the apothem for disk2 (now self) should be positive

            // Compare moments of the intersections.
            // If they are equal then it's highly probable that the intersections are equal.
            assert_eq!(polygon.moment(), result.unwrap().unwrap_left().moment());
        }
        Either::Right(_) => panic!("Expected commutative property to hold"),
    }
}
