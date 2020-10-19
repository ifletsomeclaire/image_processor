fn main() {
    editor_utils::transparency::transparency("x_image_samples/stars");
    editor_utils::read_sample_folder_and_dain_it("x_output/transparency");

    // editor_utils::read_sample_folder_and_dain_it("x_image_samples/stars");

    // edge_detector::edge_crop("x_image_samples/raw_skybox.png");
    // editor_utils::transparency::transparency("x_output/crop");
}
