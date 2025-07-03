
use std::ops::RangeInclusive;

use eframe::egui;

#[derive(Default)]
struct Application {}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(egui::Slider::new(&mut 120, RangeInclusive::new(10, 500)));
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native("hyprnome", options, Box::new(|_cc| Ok(Box::<Application>::default())))
}
