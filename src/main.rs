use std::{fs::File, io::BufReader};

use gpx::read;
use lib::segment_splitter::SegmentSplitter;

mod lib;

fn main() {
    let gpx_list: Vec<_> = [
        "tests/simple_intersection/1.gpx",
        "tests/simple_intersection/2.gpx",
    ]
    .into_iter()
    .map(|file| {
        let file = File::open(file).unwrap();
        let reader = BufReader::new(file);
        read(reader).unwrap()
    })
    .collect();
    let splitter = SegmentSplitter::from_gpx(gpx_list.iter());
    println!("{}", splitter.chunks.len());
}
