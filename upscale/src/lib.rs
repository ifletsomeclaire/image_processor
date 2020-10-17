use raster::{editor::resize, Image, InterpolationMode, ResizeMode};

pub fn raster_upscale_image(image: &mut Image, w: i32, h: i32) -> &mut Image {
    match resize(image, w, h, ResizeMode::Fill) {
        Ok(_) => {
            raster::interpolate::resample(image, w, h, InterpolationMode::Bicubic).unwrap();
            image
        }
        Err(x) => panic!("{:?}", x),
    }
}

pub fn upscale_images(images: &mut Vec<Image>, w: i32, h: i32) {
    for image in images {
        raster_upscale_image(image, w, h);
    }
}
