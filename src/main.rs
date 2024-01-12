use opencv::{highgui, imgproc, prelude::*, videoio, Result};

fn main() -> Result<()> {
    let window = "video capture";

    highgui::named_window(window, 1)?;

    let mut video = videoio::VideoCapture::new(0, videoio::CAP_FFMPEG)?; // 0 is the default camera

    video.open_file("minimal_horse.mp4", videoio::CAP_FFMPEG)?; // 0 is the default camera

    let opened = videoio::VideoCapture::is_opened(&video)?;

    if !opened {
        panic!("Unable to open default camera!");
    }

    loop {
        let mut frame = Mat::default();
        video.read(&mut frame)?;

        if frame.size()?.width > 0 {
            let mut gray = Mat::default();
            imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
            highgui::imshow(window, &gray)?;
        } else {
            video.open_file("minimal_horse.mp4", videoio::CAP_FFMPEG)?;
        }

        if highgui::wait_key(10)? > 0 {
            break;
        }
    }

    Ok(())
}
