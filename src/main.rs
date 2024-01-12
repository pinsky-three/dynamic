use std::ops::Add;

use opencv::{
    core::{add_weighted, Vector},
    highgui::{self, imshow},
    imgcodecs::imwrite,
    imgproc,
    prelude::*,
    videoio::{self, CAP_PROP_FRAME_COUNT, CAP_PROP_POS_FRAMES},
    Result,
};

fn main() -> Result<()> {
    let window = "video capture";

    highgui::named_window(window, 1)?;

    let mut video = videoio::VideoCapture::new(0, videoio::CAP_FFMPEG)?; // 0 is the default camera

    video.open_file("minimal_horse.mp4", videoio::CAP_FFMPEG)?; // 0 is the default camera

    let opened = videoio::VideoCapture::is_opened(&video)?;

    if !opened {
        panic!("Unable to open default camera!");
    }

    // let total_frames = video.get(CAP_PROP_FRAME_COUNT)? as i32;

    // create a palette from red to blue with the length of the video
    // let mut palette = Vec::with_capacity(total_frames as usize);

    let mut frames = Vec::new();

    while video.grab()? {
        let mut frame = Mat::default();

        video.read(&mut frame)?;

        // frame_counter += 1;

        // println!("frame: {}", frame_counter);

        // convert frame into a grayscale image
        let mut gray = Mat::default();
        imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

        // tint the image red
        let mut red = Mat::default();
        imgproc::cvt_color(&gray, &mut red, imgproc::COLOR_GRAY2BGR, 0)?;

        frames.push(red);
    }

    let mut sum = Mat::default();

    for i in 1..frames.len() {
        let src_1 = if i > 1 {
            sum.clone()
        } else {
            frames[i - 1].clone()
        };

        add_weighted(&src_1, 0.95, &frames[i], 0.05, 0.0, &mut sum, -1)?;
    }

    imwrite("sum.jpg", &sum, &Vector::new())?;
    // highgui::imshow(window, &sum)?;
    // highgui::wait_key(0)?;

    Ok(())
}
