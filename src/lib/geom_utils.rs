use geo::algorithm::euclidean_distance::EuclideanDistance;
use gpx::Waypoint;

use super::config::POINT_DISTANCE_THRESHOLD;

pub fn line_segments_close(p11: &Waypoint, p12: &Waypoint, p21: &Waypoint, p22: &Waypoint) -> bool {
    points_close(p11, p21)
}

pub fn points_close(p1: &Waypoint, p2: &Waypoint) -> bool {
    euclidean_distance(p1, p2) < POINT_DISTANCE_THRESHOLD
}

pub fn euclidean_distance(p1: &Waypoint, p2: &Waypoint) -> f64 {
    p1.point().euclidean_distance(&p2.point())
}
