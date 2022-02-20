use gpx::TrackSegment;

use crate::lib::geom_utils::line_segments_close;
use crate::lib::geom_utils::points_close;

use super::chunk::Chunk;
use super::config::SELF_INTERSECTION_SAFETY_DISTANCE;
use super::geom_utils::euclidean_distance;

#[derive(Debug)]
pub struct Intersection {
    pub start: usize,
    pub end: usize,
}

pub struct Segment<'a> {
    pub segment: &'a TrackSegment,
}

impl<'a> Segment<'a> {
    pub fn self_intersect(&self) -> Vec<Chunk<'a>> {
        let intersections = self.get_self_intersections();
        self.cut_into_chunks(intersections)
    }

    pub fn intersect_with(&self, other: &Segment<'a>) -> Vec<Chunk<'a>> {
        let intersections = self.get_intersections_with(other, false);
        self.cut_into_chunks(intersections)
    }

    pub fn get_self_intersections(&self) -> Vec<Intersection> {
        self.get_intersections_with(&self, true)
    }

    pub fn get_intersections_with(&self, other: &Segment<'a>, is_self: bool) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];
        for ((i, p11), p12) in self
            .segment
            .points
            .iter()
            .enumerate()
            .zip(self.segment.points[1..].iter())
        {
            let search_start = if is_self {
                // Make sure we only start searching for close points once we have found a point
                // which is far enough away from the current point (and comes after the current point
                // in the sequence)
                let start_index = other
                    .segment
                    .points
                    .iter()
                    .enumerate()
                    .find(|(j, p)| {
                        *j > i && euclidean_distance(p, p11) > SELF_INTERSECTION_SAFETY_DISTANCE
                    })
                    .map(|(j, _)| j);
                match start_index {
                    Some(start_index) => start_index,
                    None => break,
                }
            } else {
                0
            };
            let is_close = other.segment.points[search_start..]
                .iter()
                .zip(other.segment.points[search_start + 1..].iter())
                .any(|(p21, p22)| line_segments_close(p11, p12, p21, p22));
            if is_close {
                let last_intersection = intersections.last_mut();
                let mut same_intersection = false;
                if let Some(mut intersection) = last_intersection {
                    if points_close(
                        &self.segment.points[intersection.end],
                        &self.segment.points[i],
                    ) {
                        intersection.end = i;
                        same_intersection = true;
                    }
                }
                if !same_intersection {
                    intersections.push(Intersection { start: i, end: i });
                }
            }
        }
        intersections
    }

    fn cut_into_chunks(&self, intersections: Vec<Intersection>) -> Vec<Chunk<'a>> {
        dbg!(intersections);
        todo!()
    }
}
