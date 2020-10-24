use std::path::Path;

use dainargs::{DainArgs, DainNum};
use editor_utils::{image_io, upscale};

pub mod dainargs;

// TODO: expose size as setting
// TODO: also lots more settings for DAIN
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

pub fn loop_interpolation() {
    let root = std::env::current_dir().unwrap();
    let mut images = image_io::walkdir_for_images("x_output/interpolate/interpolated_frames");
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


pub fn run_dain(args: &dainargs::DainArgs) {
    std::process::Command::new("E:/DAIN Interpolation/DAIN_APP Alpha/DAINAPP")
        .args(args.to_arguments())
        .spawn()
        .expect("Error spawning DAINAPP");
}
pub fn run_dain_and_wait(args: &dainargs::DainArgs) {
    std::process::Command::new("E:/DAIN Interpolation/DAIN_APP Alpha/DAINAPP")
        .args(args.to_arguments())
        .output()
        .expect("Error outputting DAINAPP");
}
