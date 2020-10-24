use std::path::Path;

use edge_detection::{canny, Detection};

// TODO: expose sigma/threshhold settings
pub fn detect_edges<P: AsRef<Path>>(path: P) -> Detection {
    let source_image = image::open(path).expect("failed to read image").to_luma();
    let detection = canny(source_image, 2.0, 0.1, 0.01);
    detection
}

pub fn save_detection<P: AsRef<Path>>(path: P, detection: &Detection) {
    detection.as_image().save(path).unwrap();
}

pub fn gather_edge_pixels(detection: Detection) -> Vec<(i32, i32)> {
    let mut edges = Vec::new();
    for x in 0..detection.width() {
        for y in 0..detection.height() {
            if detection.interpolate(x as f32, y as f32).magnitude() > 0.0 {
                edges.push((x as i32, y as i32));
            }
        }
    }
    edges
}
