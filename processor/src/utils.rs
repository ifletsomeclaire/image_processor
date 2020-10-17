use std::path::{Path};
use gif::{Encoder, Frame, Repeat};
use interpolate::{dainargs::{DainArgs, DainBool, Interpolations}, run_dain_and_wait};
use raster::{editor::resize, Image, InterpolationMode, ResizeMode};


fn read_sample_folder_and_dain_it() {
    let root = std::env::current_dir().unwrap();
    let width = 200;
    let height = 200;

    let mut images = walkdir_for_images("image_samples");
    upscale_images(&mut images, width, height);
    save_images("x_output/image_files", &images);

    let gif_path = "x_output/gif_hack.gif";
    generate_gif(gif_path, images, width, height);

    let mut args = DainArgs::new(
        root.join(gif_path),
        root.join("x_output/interpolate"),
        String::from("gifout.gif"),
    );
    args.set_interpolations(Interpolations::Eight);
    args.set_loop(DainBool::OneTrue);
    run_dain_and_wait(&args);

    let newimages = walkdir_for_images("x_output/interpolate/interpolated_frames");
    generate_gif(gif_path, newimages, width, height);
}





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

pub fn raster_upscale_image(image: &mut Image, w: i32, h: i32) -> &mut Image {
    match resize(image, w, h, ResizeMode::Fill) {
        Ok(_) => {
            raster::interpolate::resample(image, w, h, InterpolationMode::Bicubic).unwrap();
            image
        }
        Err(x) => panic!("{:?}", x),
    }
}

pub fn upscale_images(images: &mut Vec<Image>, w: i32, h: i32) {
    for image in images {
        raster_upscale_image(image, w, h);
    }
}