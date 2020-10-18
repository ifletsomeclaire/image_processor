use std::path::Path;
use raster::{editor::crop, Image, PositionMode};

pub fn crop_by_edge_detection<P: AsRef<Path>>(path: P) {
    let detection = detect_edges(&path);
    let image = raster::open(path.as_ref().to_str().expect("path to str")).unwrap();
    let mut edges = Vec::new();
    for x in 0..detection.width() {
        for y in 0..detection.height() {
            if detection.interpolate(x as f32, y as f32).magnitude() > 0.0 {
                edges.push((x as i32, y as i32));
            }
        }
    }
    let mut crop_areas = Vec::new();
    while edges.len() > 1 {
        crop_areas.push(get_crop_area(&mut edges));
    }
    println!("{:?}", crop_areas.len());

    generate_cropped_images(image, crop_areas);
}

fn detect_edges<P: AsRef<Path>>(path: P) -> edge_detection::Detection {
    let source_image = image::open(path).expect("failed to read image").to_luma();
    let detection = edge_detection::canny(source_image, 2.0, 0.1, 0.01);
    detection
        .as_image()
        .save(&Path::new("x_output/edgy.png"))
        .unwrap();
    detection
}

fn generate_cropped_images(image: Image, crop_areas: Vec<CropArea>) {
    for (c, croparea) in crop_areas.iter().enumerate() {
        let width = (image.width).min(croparea.max_x) - 0.max(croparea.min_x);
        let height = (image.height).min(croparea.max_y) - 0.max(croparea.min_y);
        let mut im = image.clone();
        crop(
            &mut im,
            width,
            height,
            PositionMode::TopLeft,
            croparea.min_x,
            croparea.min_y,
        )
        .expect("trying to crop?");
        raster::save(&im, &format!("x_output/crop/image{}.png", c)).unwrap();
    }
}

fn get_crop_area(edges: &mut Vec<(i32, i32)>) -> CropArea {
    let pixel = edges.pop().unwrap();
    let mut crop_list = vec![pixel];

    check_nearby_pixels(pixel, edges, &mut crop_list);

    crop_list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let min_x = crop_list[0].0 as i32;
    let max_x = crop_list[crop_list.len() - 1].0 as i32;
    crop_list.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let min_y = crop_list[0].1 as i32;
    let max_y = crop_list[crop_list.len() - 1].1 as i32;

    CropArea {
        edges: crop_list,
        min_x: min_x - ((max_x - min_x) / 2) - 1,
        max_x: max_x + ((max_x - min_x) / 2) + 1,
        min_y: min_y - ((max_y - min_y) / 2) - 1,
        max_y: max_y + ((max_y - min_y) / 2) + 1,
    }
}

fn check_nearby_pixels(
    pixel: (i32, i32),
    edges: &mut Vec<(i32, i32)>,
    crop_list: &mut Vec<(i32, i32)>,
) {
    let x = pixel.0;
    let y = pixel.1;
    let mut new_list = Vec::new();

    // immediately close pixels
    search_pixel(edges, &mut new_list, (x, y - 1)); //bottom
    search_pixel(edges, &mut new_list, (x - 1, y - 1)); //bottom left
    search_pixel(edges, &mut new_list, (x - 1, y)); //left
    search_pixel(edges, &mut new_list, (x - 1, y + 1)); //top left
    search_pixel(edges, &mut new_list, (x, y + 1)); //top
    search_pixel(edges, &mut new_list, (x + 1, y + 1)); //top right
    search_pixel(edges, &mut new_list, (x + 1, y)); //right
    search_pixel(edges, &mut new_list, (x + 1, y - 1)); //bottom right

    // pixels 2 away
    search_pixel(edges, &mut new_list, (x, y - 2)); //bottom
    search_pixel(edges, &mut new_list, (x - 1, y - 2)); //bottom left
    search_pixel(edges, &mut new_list, (x - 2, y - 2)); //bottom left
    search_pixel(edges, &mut new_list, (x - 2, y - 1)); //bottom left
    search_pixel(edges, &mut new_list, (x - 2, y)); //left
    search_pixel(edges, &mut new_list, (x - 2, y + 1)); //top left
    search_pixel(edges, &mut new_list, (x - 2, y + 2)); //top left
    search_pixel(edges, &mut new_list, (x - 1, y + 2)); //top left
    search_pixel(edges, &mut new_list, (x, y + 2)); //top
    search_pixel(edges, &mut new_list, (x + 1, y + 2)); //top right
    search_pixel(edges, &mut new_list, (x + 2, y + 2)); //top right
    search_pixel(edges, &mut new_list, (x + 2, y + 1)); //top right
    search_pixel(edges, &mut new_list, (x + 2, y)); //right
    search_pixel(edges, &mut new_list, (x + 2, y - 1)); //bottom right
    search_pixel(edges, &mut new_list, (x + 2, y - 2)); //bottom right
    search_pixel(edges, &mut new_list, (x + 1, y - 2)); //bottom right


    if new_list.len() > 0 {
        for pix in new_list.iter() {
            check_nearby_pixels(*pix, edges, crop_list)
        }
        crop_list.append(&mut new_list);
    }
}

fn search_pixel(edges: &mut Vec<(i32, i32)>, new_list: &mut Vec<(i32, i32)>, pixel: (i32, i32)) {
    if let Ok(found) = edges.binary_search(&pixel) {
        new_list.push(pixel);
        edges.remove(found);
    }
}

#[derive(Debug)]
struct CropArea {
    edges: Vec<(i32, i32)>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}
