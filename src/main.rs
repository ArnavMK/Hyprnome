
use std::{thread::sleep, time::Duration};
use eframe::egui; 


struct Hyprnome{
    pub button_text: String,
    pub state: bool,
    pub bpm_value: u32 
}

impl Default for Hyprnome {
    fn default() -> Self {
        Self {
            button_text: String::from("Play"),
            state: false,
            bpm_value: 120
        }
    }
}

impl eframe::App for Hyprnome{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.vertical_centered(|ui| {

                ui.add(egui::Slider::new(&mut self.bpm_value, 10..=500));
                
                ui.add_space(20.0);

                let state_button = ui.button(&self.button_text);
                if state_button.clicked() {
                    match self.button_text.as_str() {
                        "Play" => {
                            self.button_text = String::from("Pause");
                            if self.state == false {self.state = true}

                        }
                        "Pause" => {
                            self.button_text = String::from("Play");
                            if self.state == true {self.state = false}
                        }
                        _ => ()
                    }
                }

                if self.state {
                    self.playclick(self.bpm_value);
                }

            });
        });
    }
}

impl Hyprnome {
    fn playclick(&self, bpm: u32) {
    
        let bps = bpm/60;
        let sleep_value = 1.0/f64::from(bps);
            
        println!("Click");

        sleep(Duration::from_secs_f64(sleep_value));
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native("hyprnome", options, Box::new(|_cc| Ok(Box::<Hyprnome>::default())))
}
   
