use interpolate::{dainargs::DainArgs, dainargs::DainBool, dainargs::Interpolations, run_dain_and_wait};
use upscale::upscale_images;
use utils::{generate_gif, save_images, walkdir_for_images};

mod utils;
mod upscale;

fn main() {
    read_sample_folder_and_dain_it();
}

// saving this for reference material :)
fn read_sample_folder_and_dain_it() {
    let root = std::env::current_dir().unwrap();
    let width = 200;
    let height = 200;

    let mut images = walkdir_for_images("image_samples/stars");
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
