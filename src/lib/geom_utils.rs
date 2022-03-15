use geo::algorithm::euclidean_distance::EuclideanDistance;
use gpx::Waypoint;

pub fn line_segments_distance(
    p11: &Waypoint,
    _p12: &Waypoint,
    p21: &Waypoint,
    _p22: &Waypoint,
) -> f64 {
    // Temporary, while I can't be bothered to do the proper math
    euclidean_distance(p11, p21)
}

pub fn euclidean_distance(p1: &Waypoint, p2: &Waypoint) -> f64 {
    p1.point().euclidean_distance(&p2.point())
}
