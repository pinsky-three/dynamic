use dynamic::extrusion::process_video;
use egui_extras::install_image_loaders;

use eframe::egui::{self};

fn main() -> Result<(), ()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Dynamic Extrusion",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
    .unwrap();

    Ok(())
}

struct MyApp {
    ratio: u32,
    sum_filepath: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            ratio: 9950,
            sum_filepath: "file://sum.png".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dynamic");

            ui.add(
                egui::Slider::new(&mut self.ratio, 0..=10000)
                    .step_by(1.)
                    .text("ratio"),
            );

            if ui.button("Calculate").clicked() {
                let alpha: f64 = self.ratio as f64 / 10000.;

                process_video("minimal_horse.mp4".to_string(), alpha, 1. - alpha).unwrap();
                println!("processed video");
                ctx.request_repaint();
            }

            ui.label(format!(
                "alpha: {:.4}, beta: {:.4}",
                self.ratio as f64 / 10000.,
                1. - (self.ratio as f64 / 10000.)
            ));

            let rect = ui.available_rect_before_wrap();
            let available_size = rect.size();

            let image = egui::Image::new(&self.sum_filepath).rounding(3.0);

            image.load_for_size(ctx, available_size).unwrap();

            image.paint_at(ui, rect);
        });
    }
}
