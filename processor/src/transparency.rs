use std::path::Path;

use raster::{Color, Image};

use crate::utils;

pub fn transparency_test<P: AsRef<Path>>(path: P) {
    let mut images = utils::walkdir_for_images(path);
    remove_background_noise(&mut images);
    utils::save_images("x_output/transparency", &images);
}

pub fn remove_background_noise(images: &mut Vec<Image>) {
    for image in images {
        process_pixels_transparency(image);
    }
}

fn process_pixels_transparency(image: &mut Image) {
    for x in 0..image.width {
        for y in 0..image.height {
            remove_dark_pixels(image, x, y);
            inverse_square(image, x, y);
        }
    }
}

fn remove_dark_pixels(image: &mut Image, x: i32, y: i32) {
    let p = image.get_pixel(x, y).unwrap();
    let gray = (p.r as f32 * 0.3) + (p.g as f32 * 0.59) + (p.b as f32 * 0.11);
    if gray < 10.0 {
        image
            .set_pixel(x, y, Color::rgba(p.r, p.g, p.b, 0))
            .unwrap();
    }
}
fn inverse_square(image: &mut Image, x: i32, y: i32) {
    let p = image.get_pixel(x, y).unwrap();
    let gray = (p.r as f32 * 0.3) + (p.g as f32 * 0.59) + (p.b as f32 * 0.11);
    // let inv_sqr = 1.0
    //     / distance_squared(
    //         (x as f32, y as f32),
    //         (image.width as f32 / 2.0, image.height as f32 / 2.0),
    //     );
    let alt_inv = 1.0
        / distance(
            (x as f32, y as f32),
            (image.width as f32 / 2.0, image.height as f32 / 2.0),
        ).powf(0.01);

    image
        .set_pixel(x, y, Color::rgba(p.r, p.g, p.b, (gray * alt_inv) as u8))
        .unwrap();
}

fn distance_squared(point: (f32, f32), other: (f32, f32)) -> f32 {
    (other.0 - point.0).powi(2) + (other.1 - point.1).powi(2)
}
fn distance(point: (f32, f32), other: (f32, f32)) -> f32 {
    distance_squared(point, other).sqrt()
}
