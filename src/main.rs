extern crate nokhwa;
 
use nokhwa::{Camera, CameraFormat, FrameFormat, CaptureAPIBackend};
 
fn main() {
    CaptureAPIBackend::Auto;
    let mut camera = Camera::new(
        0,
        Some(
            CameraFormat::new_from(
                640,
                480,
                FrameFormat::MJPEG,
                30
            )
        ),
    ).unwrap();
 
 
    camera.open_stream().unwrap();
    loop {
        let frame = camera.frame().unwrap();
        println!("{}, {}", frame.width(), frame.height());
    }
}
