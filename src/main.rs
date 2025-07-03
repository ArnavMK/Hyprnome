
use std::ops::RangeInclusive;
use eframe::egui;


struct Hyprnome{
    pub button_text: String
}

impl Default for Hyprnome {
    fn default() -> Self {
        Self {
            button_text: String::from("Play")
        }
    }
}

impl eframe::App for Hyprnome{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(egui::Slider::new(&mut 120, RangeInclusive::new(10, 500)));
                ui.add_space(20.0);
                let state_button = ui.button(&self.button_text);

                if state_button.clicked() {
                    match self.button_text.as_str() {
                        "Play" => self.button_text = String::from("Pause"),
                        "Pause" => self.button_text = String::from("Play"),
                        _ => ()
                    }
                }
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native("hyprnome", options, Box::new(|_cc| Ok(Box::<Hyprnome>::default())))
}
   
