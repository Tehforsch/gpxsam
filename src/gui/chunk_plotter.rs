use eframe::egui::{
    plot::{Legend, Line, Plot, Value, Values},
    Ui,
};

use crate::lib::chunk::Chunk;

pub struct ChunkPlotter {
    plot: Plot,
    lines: Vec<Line>,
}

impl ChunkPlotter {
    pub fn new() -> Self {
        let plot = Plot::new(0)
            .legend(Legend::default())
            .view_aspect(1.0)
            .show_x(false)
            .show_y(false)
            .allow_drag(false)
            .allow_zoom(false)
            .show_background(false);
        Self {
            plot,
            lines: vec![],
        }
    }

    fn get_line_for_chunk<'a>(chunk: Chunk<'a>) -> Line {
        let values_iter = chunk
            .points()
            .iter()
            .map(|point| Value::new(point.point().x(), point.point().y()));
        Line::new(Values::from_values_iter(values_iter))
    }

    pub fn add_chunk<'a>(&mut self, chunk: Chunk<'a>) {
        self.lines.push(Self::get_line_for_chunk(chunk));
    }

    pub fn show(self, ui: &mut Ui) {
        self.plot.show(ui, |plot_ui| {
            for line in self.lines {
                plot_ui.line(line);
            }
        });
    }
}
