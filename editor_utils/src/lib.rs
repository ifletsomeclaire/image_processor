use std::path::Path;

use interpolate::{dainargs::{DainArgs, DainNum}, run_dain_and_wait};

pub mod transparency;
pub mod image_io;
pub mod gif;
pub mod upscale;

pub fn read_sample_folder_and_dain_it<P: AsRef<Path>>(path: P) {
    let root = std::env::current_dir().unwrap();
    let width = 200;
    let height = 200;

    let mut images = image_io::walkdir_for_images(path);
    upscale::upscale_images(&mut images, width, height);
    images.push(images[0].clone()); // add first frame to end to ensure smooth interpolation back to start
    image_io::save_images_for_dain("x_output/interpolate/original_frames", &images);

    let mut args = DainArgs::new(
        root.join("x_output/interpolate"),
        root.join("x_output/interpolate"),
        String::from("interp_out.mp4"),
    );
    args.set_interpolations(DainNum::Eight);
    args.set_loop(DainNum::OneTrue);
    args.set_alpha(DainNum::Two);
    run_dain_and_wait(&args);
}



