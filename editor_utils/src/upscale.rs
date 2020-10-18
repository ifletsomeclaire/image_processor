use raster::{Image, InterpolationMode};
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};

pub fn upscale_images(images: &mut Vec<Image>, w: i32, h: i32) {
    images.par_iter_mut().for_each(|image| {
        raster::interpolate::resample(image, w, h, InterpolationMode::Bicubic).unwrap()
    });
}
