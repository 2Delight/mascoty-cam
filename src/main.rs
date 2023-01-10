// extern crate nokhwa;

use nokhwa::backends::capture::AVFoundationCaptureDevice;
use nokhwa::query;
use nokhwa::utils::Resolution;
use nokhwa::utils::{ApiBackend, CameraFormat, FrameFormat, RequestedFormat, RequestedFormatType};
use nokhwa::Camera;
use nokhwa::pixel_format::RgbFormat;

fn main() {
    let cams = query(ApiBackend::Auto).unwrap();
    println!("{}", cams.len());
    println!("{}", cams[0].index());

    if cams.len() == 0 {
        panic!("OOF");
    }

    let format_type = RequestedFormatType::Exact(CameraFormat::new_from(1280, 720, FrameFormat::MJPEG, 30));
    let form = RequestedFormat::new::<RgbFormat>(format_type);

    let mut camera = Camera::new(cams[0].index().to_owned(), form).unwrap();

    println!("{}", camera.info());

    camera.open_stream().unwrap();

    let frame = camera.frame().unwrap();
    println!(
        "{}, {}",
        frame.resolution().x(),
        frame.resolution().y(),
    );
}
