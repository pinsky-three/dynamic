use dynamic::extrusion::process_video;
use egui_extras::install_image_loaders;

use eframe::egui;

fn main() -> Result<(), ()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            // egui_extras::install_image_loaders(&cc.egui_ctx);

            install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
    .unwrap();

    Ok(())
}

struct MyApp {
    name: String,
    age: u32,
    sum_filepath: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 900,
            sum_filepath: "file://sum.png".to_string(),
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
            ui.add(egui::Slider::new(&mut self.age, 0..=1000).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;

                let alpha: f64 = self.age as f64 / 1000.;

                process_video("minimal_horse.mp4".to_string(), alpha, 1. - alpha).unwrap();
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));

            ui.image(&self.sum_filepath);
        });
    }
}
