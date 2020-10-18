use std::path::Path;

use interpolate::{
    dainargs::{DainArgs, DainBool, Interpolations},
    run_dain_and_wait,
};

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
    image_io::save_images("x_output/image_files", &images);

    let gif_path = "x_output/gif_hack.gif";
    gif::generate_gif(gif_path, images, width, height);

    let mut args = DainArgs::new(
        root.join(gif_path),
        root.join("x_output/interpolate"),
        String::from("gifout.gif"),
    );
    args.set_interpolations(Interpolations::Eight);
    args.set_loop(DainBool::OneTrue);
    run_dain_and_wait(&args);

    let newimages = image_io::walkdir_for_images("x_output/interpolate/interpolated_frames");
    gif::generate_gif(gif_path, newimages, width, height);
}



