use std::{fs::File, io::BufReader};

use gpx::read;
use gui::Gui;

mod gui;
mod lib;

fn main() {
    let gpx_list: Vec<_> = [
        "tests/simple_intersection_2/1.gpx",
        "tests/simple_intersection_2/2.gpx",
        "tests/simple_intersection_2/3.gpx",
    ]
    .into_iter()
    .map(|file| {
        let file = File::open(file).unwrap();
        let reader = BufReader::new(file);
        read(reader).unwrap()
    })
    .collect();
    let app = Gui::new(&gpx_list);
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
