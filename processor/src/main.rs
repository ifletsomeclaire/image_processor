use interpolate::{dainargs::DainArgs, dainargs::DainBool, dainargs::Interpolations, run_dain_and_wait};
use utils::{generate_gif, save_images, upscale_images, walkdir_for_images};

mod utils;

fn main() {
    edge_detector::detect_edges();
}
