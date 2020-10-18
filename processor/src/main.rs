
mod transparency;
mod utils;

fn main() {
    edge_detector::crop_by_edge_detection("image_samples/raw_skybox.png");
    transparency::transparency("x_output/crop");
}
