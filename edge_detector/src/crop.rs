use edge_detection::Detection;
use raster::{editor::crop, Image, PositionMode};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::edge::gather_edge_pixels;

pub fn crop_by_edge_detection(image: Image, detection: Detection) -> Vec<Image> {
    let mut edges = gather_edge_pixels(detection);
    let mut crop_areas = Vec::new();
    while edges.len() > 1 {
        crop_areas.push(get_crop_area(&mut edges)); // TODO find a way to put this in rayon?
    }
    println!("{:?}", crop_areas.len());
    generate_cropped_images(image, crop_areas)
}

fn generate_cropped_images(image: Image, crop_areas: Vec<CropArea>) -> Vec<Image> {
    crop_areas.par_iter().map(|croparea| {
        // TODO: create size function to determine these properly; incorporate min/max processing from below
        // Maybe also include settings that would make image tighter in or wider out?
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
        im
    }).collect::<Vec<_>>()
}

// TODO try to make this more efficient; popping and removing from Vec is not
fn get_crop_area(edges: &mut Vec<(i32, i32)>) -> CropArea {
    let pixel = edges.pop().unwrap();
    let mut crop_list = vec![pixel];

    check_nearby_pixels(pixel, edges, &mut crop_list);

    // TODO: is this really the best way?
    crop_list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let min_x = crop_list[0].0 as i32;
    let max_x = crop_list[crop_list.len() - 1].0 as i32;
    crop_list.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let min_y = crop_list[0].1 as i32;
    let max_y = crop_list[crop_list.len() - 1].1 as i32;

    // TODO simplify here, and add min/max processing elsewhere
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
