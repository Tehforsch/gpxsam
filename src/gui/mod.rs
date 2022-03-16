use eframe::egui::{self};
use eframe::epi;
use gpx::Gpx;

mod chunk_plotter;

use crate::lib::chunk::Chunk;
use crate::lib::segment_splitter::SegmentSplitter;

use self::chunk_plotter::ChunkPlotter;

pub struct Gui {
    files: Vec<Gpx>,
}

impl Gui {
    pub fn new(files: &[Gpx]) -> Gui {
        Gui {
            files: files.to_vec(),
        }
    }
}

impl epi::App for Gui {
    fn name(&self) -> &str {
        "Gpxsam"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _: &epi::Frame) {
        let splitter = SegmentSplitter::from_gpx(self.files.iter());
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut plot = ChunkPlotter::new();
            // for f in self.files.iter() {
            //     for track in f.tracks.iter() {
            //         for segment in track.segments.iter() {
            //             let chunk = Chunk::from_entire_segment(segment);
            //             plot.add_chunk(chunk)
            //         }
            //     }
            // }
            for chunk in splitter.chunks {
                println!("{} {}", chunk.start, chunk.end);
                plot.add_chunk(chunk)
            }
            plot.show(ui);
        });
    }
}
