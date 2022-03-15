use gpx::Gpx;

use super::chunk::Chunk;

pub struct SegmentSplitter<'a> {
    pub segments: Vec<Chunk<'a>>,
}

impl<'a> SegmentSplitter<'a> {
    pub fn from_gpx(gpx_list: impl Iterator<Item = &'a Gpx>) -> SegmentSplitter<'a> {
        let mut splitter = SegmentSplitter { segments: vec![] };
        for gpx in gpx_list {
            splitter.add_gpx(gpx)
        }
        splitter
    }

    fn add_gpx(&mut self, gpx: &'a Gpx) {
        for track in gpx.tracks.iter() {
            for segment in track.segments.iter() {
                self.segments.push(Chunk::from_entire_segment(segment))
            }
        }
    }

    pub fn split(&mut self) -> Vec<Chunk<'_>> {
        let mut chunks = vec![];
        for segment in self.segments.iter() {
            let new_chunks = segment.self_intersect();
            for chunk in new_chunks {
                Self::add_chunk(&mut chunks, chunk);
            }
        }
        chunks
    }

    pub fn add_chunk<'b>(chunks: &mut Vec<Chunk<'b>>, chunk: Chunk<'b>) {
        chunks.push(chunk);
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use gpx::read;

    use super::SegmentSplitter;

    // #[test]
    // fn test_simple_intersection() {
    //     test_splitting(&[
    //         "tests/simple_intersection/1.gpx",
    //         "tests/simple_intersection/2.gpx",
    //     ], 4);
    // }

    #[test]
    fn test_self_intersection() {
        test_splitting(&["tests/self_intersection/1.gpx"], 3);
    }

    fn test_splitting(files: &[&str], num_chunks_desired: usize) {
        let gpx_list: Vec<_> = files
            .into_iter()
            .map(|file| {
                let file = File::open(file).unwrap();
                let reader = BufReader::new(file);
                read(reader).unwrap()
            })
            .collect();
        let mut splitter = SegmentSplitter::from_gpx(gpx_list.iter());
        assert_eq!(splitter.split().len(), num_chunks_desired)
    }
}
