#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::sync::Arc;
use eframe::egui;
use eframe::egui::{Color32, ColorImage, Frame, ImageData, ImageSource, Pos2, pos2, TextureHandle, TextureId, TextureOptions};
use eframe::egui::load::Bytes;
use eframe::emath::Rect;
use eframe::epaint::TextureManager;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            let ctx = ui.painter().ctx();

            let img = ColorImage {
                size: [100, 100],
                pixels: vec![Color32::from_rgb(0, 255, 156); 100*100]
            };

            let im = ImageData::Color(Arc::new(img));
            let mut mgr = TextureManager::default();

            let id = mgr.alloc("buf".into(), im, TextureOptions::default());

            ui.painter().image(id, eframe::egui::Rect::from_min_max(pos2(50.0, 0.0), pos2(100.0, 100.0)), eframe::egui::Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), eframe::egui::Color32::WHITE);
        });
    }
}