use std::path::Path;

pub fn detect_edges() {
    let source_image = image::open("E:/Rust/Projects/dark_sky_editor/assets/STSCI-H-p1917b-q-5198x4801.png")
        .expect("failed to read image")
        .to_luma();
    let detection = edge_detection::canny(
        source_image,
        0.8,  // sigma
        0.1,  // strong threshold
        0.01, // weak threshold
    );
    let im = detection.as_image();
    im.save(&Path::new("x_output/edgy.png")).unwrap();
}

