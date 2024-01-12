use opencv::{
    highgui, imgproc,
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

    let mut frame_counter = 0;

    loop {
        let mut frame = Mat::default();
        video.read(&mut frame)?;
        frame_counter += 1;

        if frame.size()?.width > 0 {
            let mut gray = Mat::default();
            imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
            highgui::imshow(window, &gray)?;
        }

        if frame_counter == video.get(CAP_PROP_FRAME_COUNT)? as i32 {
            frame_counter = 0;

            video.set(CAP_PROP_POS_FRAMES, 0.0)?;
        }

        if highgui::wait_key(10)? > 0 {
            break;
        }
    }

    Ok(())
}
