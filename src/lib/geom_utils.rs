use geo::algorithm::euclidean_distance::EuclideanDistance;
use gpx::Waypoint;

pub fn line_segments_distance(
    p11: &Waypoint,
    p12: &Waypoint,
    p21: &Waypoint,
    p22: &Waypoint,
) -> f64 {
    // Temporary, while I can't be bothered to do the proper math
    euclidean_distance(p11, p21)
        .min(euclidean_distance(p11, p22))
        .min(euclidean_distance(p12, p21))
        .min(euclidean_distance(p12, p22))
}

pub fn euclidean_distance(p1: &Waypoint, p2: &Waypoint) -> f64 {
    p1.point().euclidean_distance(&p2.point())
}
