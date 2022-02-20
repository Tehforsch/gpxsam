use gpx::TrackSegment;

pub struct Chunk<'a> {
    pub parent: &'a TrackSegment,
    pub start: usize,
    pub end: usize,
}
