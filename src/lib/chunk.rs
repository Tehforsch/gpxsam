use super::segment::Segment;

pub struct Chunk<'a> {
    pub parent: Segment<'a>,
}
