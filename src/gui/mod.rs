use eframe::egui::{self};
use eframe::epi;
use gpx::Gpx;

mod chunk_plotter;

use crate::lib::chunk::Chunk;
use crate::lib::segment_splitter::SegmentSplitter;

use self::chunk_plotter::ChunkPlotter;

pub struct Gui {
    files: Vec<Gpx>,
    show_files: bool,
}

impl Gui {
    pub fn new(files: &[Gpx]) -> Gui {
        Gui {
            files: files.to_vec(),
            show_files: false,
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
            ui.checkbox(&mut self.show_files, "Show files");
            let mut plot = ChunkPlotter::new();
            if self.show_files {
                for file in self.files.iter() {
                    for track in file.tracks.iter() {
                        for segment in track.segments.iter() {
                            plot.add_chunk(Chunk::from_entire_segment(segment));
                        }
                    }
                }
            } else {
                for chunk in splitter.chunks {
                    plot.add_chunk(chunk)
                }
            }
            plot.show(ui);
        });
    }
}
