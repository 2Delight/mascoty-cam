extern crate nokhwa;
 
use nokhwa::*;
 
fn main() {
    let cams = query_devices(CaptureAPIBackend::Auto).unwrap();
    println!("{}", cams.len());
    println!("{}", cams[0].index());
 
    if cams.len() == 0 {
        println!("OOF");
    }
 
    let mut camera = Camera::new(
        cams[0].index(),
        Some(CameraFormat::new_from(640, 480, FrameFormat::MJPEG, 30)),
    )
    .unwrap();
 
    println!("{}", camera.info());
 
    camera.open_stream().unwrap();
 
    let frame = camera.frame().unwrap();
    println!("{}, {}, {:?}", frame.width(), frame.height(), frame.get_pixel(10, 10).0);
}
