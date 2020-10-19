use raster::{Color, Image};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use std::path::Path;

use crate::image_io;

pub fn transparency<P: AsRef<Path>>(path: P) {
    let mut images = image_io::walkdir_for_images(path);
    remove_background_noise(&mut images);
    image_io::save_images("x_output/transparency", &images);
}

pub fn remove_background_noise(images: &mut Vec<Image>) {
    images.par_iter_mut().for_each(|image| {
        image.remove_dark_pixels(20.0);
        image.remove_by_inverse_square(0.01)
    });
}

pub trait Transparency {
    fn remove_dark_pixels(&mut self, max_alpha: f32);
    fn remove_by_inverse_square(&mut self, square: f32);
}

impl Transparency for Image {
    fn remove_dark_pixels(&mut self, max_alpha: f32) {
        for x in 0..self.width {
            for y in 0..self.height {
                let p = self.get_pixel(x, y).unwrap();
                let gray = (p.r as f32 * 0.3) + (p.g as f32 * 0.59) + (p.b as f32 * 0.11);
                if gray < max_alpha {
                    self.set_pixel(x, y, Color::rgba(0, 0, 0, 0)).unwrap();
                }
            }
        }
    }
    fn remove_by_inverse_square(&mut self, square: f32) {
        let center = (self.width as f32 / 2.0, self.height as f32 / 2.0);
        let dead_zone = distance((self.width as f32 / 2.5, self.height as f32 / 2.5), center);
        for x in 0..self.width {
            for y in 0..self.height {
                let p = self.get_pixel(x, y).unwrap();
                let gray = (p.r as f32 * 0.33) + (p.g as f32 * 0.33) + (p.b as f32 * 0.33);
                let distance = distance(
                    (x as f32, y as f32),
                    center,
                );
                if distance > dead_zone {
                    let alt_inv = 1.0 / (distance - dead_zone).powf(square);
                    self.set_pixel(x, y, Color::rgba(p.r, p.g, p.b, (gray * alt_inv) as u8))
                        .unwrap();
                }
            }
        }
    }
}

fn distance_squared(point: (f32, f32), other: (f32, f32)) -> f32 {
    (other.0 - point.0).powi(2) + (other.1 - point.1).powi(2)
}
fn distance(point: (f32, f32), other: (f32, f32)) -> f32 {
    distance_squared(point, other).sqrt()
}
