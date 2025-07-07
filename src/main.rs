use std::{
    fs::File,
    io::BufReader,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
    time::Duration,
};

use eframe::egui;
use rodio::{buffer::SamplesBuffer, source::Source, Decoder, OutputStream, OutputStreamHandle};

pub struct Hyprnome {
    button_text: String,
    bpm_value: u32,
    is_running: bool,

    #[allow(dead_code)]
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    click_sound: SamplesBuffer<f32>,

    ticking: Arc<AtomicBool>,
    tick_handle: Option<JoinHandle<()>>,
}

impl Default for Hyprnome {
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        let file = BufReader::new(File::open("/home/arnav/Audio/MetronomeClickSound.ogg").unwrap());
        let source = Decoder::new(file).unwrap().convert_samples::<f32>();

        let channels = source.channels();
        let sample_rate = source.sample_rate();
        let data: Vec<f32> = source.collect();

        let click_sound = SamplesBuffer::new(channels, sample_rate, data);

        Self {
            button_text: "Play".into(),
            bpm_value: 121,
            is_running: false,
            stream,
            stream_handle,
            click_sound,
            ticking: Arc::new(AtomicBool::new(false)),
            tick_handle: None,
        }
    }
}

impl eframe::App for Hyprnome {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(egui::Slider::new(&mut self.bpm_value, 70..=500));
                ui.add_space(20.0);

                if ui.button(&self.button_text).clicked() {
                    match self.button_text.as_str() {
                        "Play" => {
                            self.button_text = "Pause".into();
                            self.is_running = true;

                            let bpm = self.bpm_value;
                            let flag = self.ticking.clone();
                            let sound = self.click_sound.clone();
                            let handle = self.stream_handle.clone();

                            flag.store(true, Ordering::SeqCst);

                            self.tick_handle = Some(thread::spawn(move || {
                                let sleep_time = Duration::from_secs_f64(60.0 / bpm as f64);
                                while flag.load(Ordering::SeqCst) {
                                    let _ = handle.play_raw(sound.clone().convert_samples());
                                    thread::sleep(sleep_time);
                                }
                            }));
                        }
                        "Pause" => {
                            self.button_text = "Play".into();
                            self.is_running = false;
                            self.ticking.store(false, Ordering::SeqCst);

                            if let Some(handle) = self.tick_handle.take() {
                                let _ = handle.join();
                            }
                        }
                        _ => {}
                    }
                }
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Ensure clean shutdown
        self.ticking.store(false, Ordering::SeqCst);
        if let Some(handle) = self.tick_handle.take() {
            let _ = handle.join();
        }
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native("hyprnome", options, Box::new(|_cc| Ok(Box::<Hyprnome>::default())))
}
   
