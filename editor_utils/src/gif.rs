use std::path::Path;
use gif::{Encoder, Frame, Repeat};
use raster::Image;

// TODO: figure out how to make it work with transparent images?
pub fn generate_gif<P: AsRef<Path>>(path: P, images: Vec<Image>, w: i32, h: i32) {
    let mut gif = std::fs::File::create(&path).unwrap();
    let mut encoder = Encoder::new(&mut gif, w as u16, h as u16, &[]).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for mut image in images {
        let frame = Frame::from_rgba(image.width as u16, image.height as u16, &mut image.bytes);
        encoder.write_frame(&frame).unwrap();
    }
}
