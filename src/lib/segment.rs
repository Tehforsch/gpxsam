use std::iter;

use gpx::TrackSegment;
use gpx::Waypoint;

use super::chunk::Chunk;
use super::config::CHUNK_DISTANCE_THRESHOLD;
use super::config::POINT_DISTANCE_THRESHOLD;
use super::config::SELF_INTERSECTION_SAFETY_DISTANCE;
use super::geom_utils::euclidean_distance;
use super::geom_utils::line_segments_distance;

#[derive(Debug)]
pub struct Intersection {
    pub start: usize,
    pub end: usize,
}

pub struct Segment<'a> {
    pub segment: &'a TrackSegment,
}

impl<'a> Segment<'a> {
    pub fn self_intersect<'b>(&'b self) -> Vec<Chunk<'b>> {
        let intersections = self.get_intersections_with(&self, true);
        self.cut_into_chunks(intersections)
    }

    pub fn intersect_with<'b>(&'b self, other: &Segment<'a>) -> Vec<Chunk<'b>> {
        let intersections = self.get_intersections_with(other, false);
        self.cut_into_chunks(intersections)
    }

    pub fn get_intersections_with(
        &self,
        other: &Segment<'a>,
        other_is_self: bool,
    ) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = vec![];
        let average_neighbour_distance = self.get_average_distance_next_point();
        let point_threshold = POINT_DISTANCE_THRESHOLD * average_neighbour_distance;
        let safety_threshold = SELF_INTERSECTION_SAFETY_DISTANCE * average_neighbour_distance;
        for ((i, p11), p12) in self
            .segment
            .points
            .iter()
            .enumerate()
            .zip(self.segment.points[1..].iter())
        {
            let mut search_lines = if other_is_self {
                // Make sure we only search for close points within the points that have moved
                // far enough away from the current point
                other.get_lines_at_safety_distance(i, safety_threshold)
            } else {
                other.get_lines()
            };
            let is_close = search_lines
                .any(|(p21, p22)| line_segments_distance(p11, p12, p21, p22) < point_threshold);
            if is_close {
                let last_intersection = intersections.last_mut();
                let mut same_intersection = false;
                if let Some(mut intersection) = last_intersection {
                    if euclidean_distance(
                        &self.segment.points[intersection.end],
                        &self.segment.points[i],
                    ) < point_threshold
                    {
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

    fn get_lines<'b>(&'b self) -> Box<dyn Iterator<Item = (&Waypoint, &Waypoint)> + 'b> {
        Box::new(
            self.segment.points[..]
                .iter()
                .zip(self.segment.points[1..].iter()),
        )
    }

    fn get_lines_at_safety_distance<'b>(
        &'b self,
        i: usize,
        safety_threshold: f64,
    ) -> Box<dyn Iterator<Item = (&Waypoint, &Waypoint)> + 'b> {
        let start_index_after = self
            .segment
            .points
            .iter()
            .enumerate()
            .filter(|(j, p)| {
                *j > i && euclidean_distance(p, &self.segment.points[i]) > safety_threshold
            })
            .map(|(j, _)| j)
            .min();
        let end_index_before = self
            .segment
            .points
            .iter()
            .enumerate()
            .filter(|(j, p)| {
                *j < i && euclidean_distance(p, &self.segment.points[i]) > safety_threshold
            })
            .map(|(j, _)| j)
            .max();
        let mut slice_before = match end_index_before {
            Some(end_index_before) => self.segment.points[0..end_index_before]
                .iter()
                .zip(self.segment.points[1..end_index_before].iter()),
            // Uglily construct an empty slice to create the same type
            None => [].iter().zip(&[]),
        };

        let mut slice_after = match start_index_after {
            Some(start_index_after) => self.segment.points[start_index_after..]
                .iter()
                .zip(self.segment.points[start_index_after + 1..].iter()),
            // Uglily construct an empty slice
            None => [].iter().zip(&[]),
        };
        Box::new(slice_before.chain(slice_after))
    }

    fn cut_into_chunks(&'a self, mut intersections: Vec<Intersection>) -> Vec<Chunk<'a>> {
        let average_neighbour_distance = self.get_average_distance_next_point();
        let significant_chunk_threshold = CHUNK_DISTANCE_THRESHOLD * average_neighbour_distance;
        let start = Intersection { start: 0, end: 0 };
        let end = Intersection {
            start: self.segment.points.len() - 1,
            end: self.segment.points.len() - 1,
        };
        intersections.insert(0, start);
        intersections.push(end);
        let mut chunks = vec![];
        let mut add_if_length_over_threshold = |start, end| {
            let length = self.length_between(start, end);
            if length > significant_chunk_threshold {
                chunks.push(Chunk {
                    parent: self.segment,
                    start,
                    end,
                });
            }
        };
        for (i1, i2) in intersections.iter().zip(intersections[1..].iter()) {
            add_if_length_over_threshold(i1.start, i1.end); // Add the intersection itself if it is long enough
            add_if_length_over_threshold(i1.end, i2.start); // Add the chunk in between if it is long enough
        }
        chunks
    }

    fn length_between(&self, start: usize, end: usize) -> f64 {
        if start == end {
            return 0.0;
        }
        self.segment.points[start..end]
            .iter()
            .zip(self.segment.points[start + 1..end].iter())
            .map(|(p1, p2)| euclidean_distance(p1, p2))
            .sum::<f64>()
    }

    pub fn get_average_distance_next_point(&self) -> f64 {
        let num_points = self.segment.points.len();
        self.length_between(0, num_points) / (num_points - 1) as f64
    }
}
