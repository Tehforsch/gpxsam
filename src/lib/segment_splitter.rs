use gpx::Gpx;

use super::chunk::Chunk;

pub struct SegmentSplitter<'a> {
    pub chunks: Vec<Chunk<'a>>,
}

impl<'a> SegmentSplitter<'a> {
    pub fn from_gpx(gpx_list: impl Iterator<Item = &'a Gpx>) -> SegmentSplitter<'a> {
        let mut splitter = SegmentSplitter { chunks: vec![] };
        for gpx in gpx_list {
            splitter.add_gpx(gpx)
        }
        splitter
    }

    fn add_gpx(&mut self, gpx: &'a Gpx) {
        for track in gpx.tracks.iter() {
            for segment in track.segments.iter() {
                self.add_chunk(Chunk::from_entire_segment(segment));
            }
        }
    }

    pub fn add_chunk(&mut self, new_chunk: Chunk<'a>) {
        let new_chunks = new_chunk.self_intersect();
        for chunk in new_chunks {
            self.add_self_intersection_free_chunk(chunk);
        }
    }

    pub fn add_self_intersection_free_chunk(&mut self, new_chunk: Chunk<'a>) {
        self.chunks.push(new_chunk);
        self.split_at_all_intersections();
    }

    pub fn split_at_all_intersections(&mut self) {
        while {
            let mut found_any_intersections = false;
            let num_chunks = self.chunks.len();
            for (i, j) in
                (0..num_chunks).flat_map(move |i| ((i + 1)..num_chunks).map(move |j| (i, j)))
            {
                println!("Checking {} vs {}", i, j);
                // We don't do any self intersection checks here anymore.
                if self.chunks[i].parent == self.chunks[j].parent {
                    continue;
                }
                let intersections1 = self.chunks[i].get_intersections_with(&self.chunks[j], false);
                let intersections2 = self.chunks[j].get_intersections_with(&self.chunks[i], false);
                if !intersections1.is_empty() {
                    found_any_intersections = true;
                    let chunk2 = self.chunks.remove(j);
                    let chunk1 = self.chunks.remove(i);
                    let new_chunks1 = chunk1.cut_into_chunks(intersections1);
                    let new_chunks2 = chunk2.cut_into_chunks(intersections2);
                    for chunk in new_chunks1 {
                        self.chunks.push(chunk);
                    }
                    for chunk in new_chunks2 {
                        self.chunks.push(chunk);
                    }
                    break;
                }
            }
            found_any_intersections
        } {}
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use gpx::read;

    use super::SegmentSplitter;

    #[test]
    fn test_simple_intersection_1() {
        test_splitting(
            &[
                "tests/simple_intersection_1/1.gpx",
                "tests/simple_intersection_1/2.gpx",
            ],
            4,
        );
    }

    #[test]
    fn test_simple_intersection_2() {
        test_splitting(
            &[
                "tests/simple_intersection_2/1.gpx",
                "tests/simple_intersection_2/2.gpx",
                "tests/simple_intersection_2/3.gpx",
            ],
            7,
        );
    }

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
        let splitter = SegmentSplitter::from_gpx(gpx_list.iter());
        assert_eq!(splitter.chunks.len(), num_chunks_desired)
    }
}
