use std::path::{Path};
use gif::{Encoder, Frame, Repeat};
use raster::Image;

pub fn walkdir_for_images<P: AsRef<Path>>(path: P) -> Vec<Image> {
    let mut images = Vec::new();
    for entry in walkdir::WalkDir::new(path) {
        match entry {
            Ok(direntry) => {
                let path = direntry.into_path();
                if let Some(ext) = path.extension() {
                    if ext.to_str().expect("expect OsStr to str") == "png" {
                        images.push(
                            raster::open(path.to_str().expect("expect PathBuf to str")).unwrap(),
                        );
                    }
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
    images
}

pub fn generate_gif<P: AsRef<Path>>(path: P, images: Vec<Image>, w: i32, h: i32) {
    let mut gif = std::fs::File::create(&path).unwrap();
    let mut encoder = Encoder::new(&mut gif, w as u16, h as u16, &[]).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for mut image in images {
        let frame = Frame::from_rgba(image.width as u16, image.height as u16, &mut image.bytes);
        encoder.write_frame(&frame).unwrap();
    }
}

pub fn save_images(path: &str, images: &Vec<Image>) {
    for (i, image) in images.iter().enumerate() {
        let _ = raster::save(&image, &format!("{}/image{}.png", path, i));
    }
}
