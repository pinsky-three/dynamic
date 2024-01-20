use egui_extras::install_image_loaders;
use opencv::core::Scalar;

use opencv::{
    core::{add_weighted, Vector},
    // highgui::{self},
    imgcodecs::imwrite,
    imgproc,
    prelude::*,
    videoio::{self},
    Result,
};

use eframe::egui;

fn main() -> Result<()> {
    // let window = "video capture";

    // highgui::named_window(window, 1)?;

    let mut video = videoio::VideoCapture::new(0, videoio::CAP_FFMPEG)?; // 0 is the default camera

    video.open_file("minimal_horse.mp4", videoio::CAP_FFMPEG)?; // 0 is the default camera

    let opened = videoio::VideoCapture::is_opened(&video)?;

    if !opened {
        panic!("Unable to open default camera!");
    }

    let mut frames = Vec::new();

    while video.grab()? {
        let mut frame = Mat::default();

        video.read(&mut frame)?;

        frames.push(frame);
    }

    let total_frames = frames.len() as f64;

    frames = frames
        .iter()
        .enumerate()
        .map(|(i, frame)| {
            // let mut gray = Mat::default();
            // imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();

            // let mut post_processed_frame = Mat::default();
            // imgproc::cvt_color(&gray, &mut post_processed_frame, imgproc::COLOR_GRAY2BGR, 0)
            //     .unwrap();

            let mut post_processed_frame = frame.clone();

            let (r, g, b) = hsv_to_rgb(360. * i as f64 / total_frames, 1.0, 1.0);

            let scalar_color =
                Scalar::new(b as f64 / 255.0, g as f64 / 255.0, r as f64 / 255.0, 1.0);

            post_processed_frame = post_processed_frame
                .mul(&scalar_color, 1.0)
                .unwrap()
                .to_mat()
                .unwrap();

            post_processed_frame
        })
        .collect();

    let mut sum = Mat::default();

    sum.set_scalar(Scalar::new(1.0, 1.0, 1.0, 1.0))?;

    for i in 1..frames.len() {
        let src_1 = if i > 1 {
            sum.clone()
        } else {
            frames[i - 1].clone()
        };

        add_weighted(&src_1, 0.99, &frames[i], 0.05, 0.0, &mut sum, -1)?;

        let img_name = format!("frames/frame_{}.jpg", i);
        imwrite(img_name.as_str(), &frames[i - 1], &Vector::new())?;
    }

    imwrite("sum.png", &sum, &Vector::new())?;

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

fn hsv_to_rgb(hue: f64, saturation: f64, value: f64) -> (u8, u8, u8) {
    fn is_between(value: f64, min: f64, max: f64) -> bool {
        min <= value && value < max
    }

    check_bounds(hue, saturation, value);

    let c = value * saturation;
    let h = hue / 60.0;
    let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
    let m = value - c;

    let (r, g, b): (f64, f64, f64) = if is_between(h, 0.0, 1.0) {
        (c, x, 0.0)
    } else if is_between(h, 1.0, 2.0) {
        (x, c, 0.0)
    } else if is_between(h, 2.0, 3.0) {
        (0.0, c, x)
    } else if is_between(h, 3.0, 4.0) {
        (0.0, x, c)
    } else if is_between(h, 4.0, 5.0) {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

fn check_bounds(hue: f64, saturation: f64, value: f64) {
    fn panic_bad_params(name: &str, from_value: &str, to_value: &str, supplied: f64) -> ! {
        panic!(
            "param {} must be between {} and {} inclusive; was: {}",
            name, from_value, to_value, supplied
        )
    }

    // println!("hue: {}", hue);

    if !(0.0..=360.0).contains(&hue) {
        panic_bad_params("hue", "0.0", "360.0", hue)
    } else if !(0.0..=1.0).contains(&saturation) {
        panic_bad_params("saturation", "0.0", "1.0", saturation)
    } else if !(0.0..=1.0).contains(&value) {
        panic_bad_params("value", "0.0", "1.0", value)
    }
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
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));

            ui.image("file://sum.png");
        });
    }
}
