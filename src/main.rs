use dynamic::extrusion::process_video;
use egui_extras::install_image_loaders;

use eframe::egui;

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
            ratio: 990,
            sum_filepath: "file://sum.png".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            println!("Hello World!");

            ui.heading("Dynamic");
            // ui.horizontal(|ui| {
            //     let name_label = ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name)
            //         .labelled_by(name_label.id);
            // });
            ui.add(egui::Slider::new(&mut self.ratio, 0..=1000).text("ratio"));
            if ui.button("Calculate").clicked() {
                let alpha: f64 = self.ratio as f64 / 1000.;

                process_video("minimal_horse.mp4".to_string(), alpha, 1. - alpha).unwrap();
                println!("processed video");
            }
            ui.label(format!(
                "alpha: {:.3}, beta: {:.3}",
                self.ratio as f64 / 1000.,
                1. - (self.ratio as f64 / 1000.)
            ));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));

            // ui.image(&self.sum_filepath);
            ui.add(egui::Image::from_uri(&self.sum_filepath).shrink_to_fit());
        });
    }
}
