use gpx::TrackSegment;

use super::chunk::Chunk;

pub struct Segment<'a> {
    pub segment: &'a TrackSegment,
}

impl<'a> Segment<'a> {
    pub fn self_intersect(&self) -> Vec<Chunk<'a>> {
        todo!()
    }
}
