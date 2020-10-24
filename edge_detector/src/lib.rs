use crop::crop_by_edge_detection;
use edge::{detect_edges, save_detection};
use std::path::Path;

pub mod crop;
pub mod edge;

pub fn edge_crop<P: AsRef<Path>>(path: P) {
    let detection = detect_edges(&path);
    save_detection("x_output/edgy.png", &detection);
    let images = crop_by_edge_detection(
        raster::open(path.as_ref().to_str().expect("path to str")).unwrap(),
        detection,
    );
    editor_utils::image_io::save_images("x_output/crop", &images);
}
