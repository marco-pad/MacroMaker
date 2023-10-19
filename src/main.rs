#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "MarcoPad Macromaker",
        options,
        Box::new(|_cc| {
            // This gives us image support:

            Box::<App>::default()
        }),
    )
}

#[derive(Default)]
struct App {}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| ui.heading("MarcoPad"));
    }
}
