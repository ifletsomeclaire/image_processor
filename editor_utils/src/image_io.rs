use std::path::Path;
use raster::Image;

// walkdir is recursive; ensure there are no nested folders in path provided
pub fn walkdir_for_images<P: AsRef<Path>>(path: P) -> Vec<Image> {
    let mut images = Vec::new();
    for entry in walkdir::WalkDir::new(path) {
        match entry {
            Ok(direntry) => {
                let path = direntry.into_path();
                if let Some(ext) = path.extension() {
                    if ext.to_str().expect("expect OsStr to str") == "png" {
                        images.push(
                            raster::open(path.to_str().expect("expect PathBuf to str")).unwrap(),
                        );
                    }
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
    images
}


pub fn save_images(path: &str, images: &Vec<Image>) {
    for (i, image) in images.iter().enumerate() {
        let _ = raster::save(&image, &format!("{}/image{}.png", path, i));
    }
}
pub fn save_images_for_dain(path: &str, images: &Vec<Image>) {
    for (i, image) in images.iter().enumerate() {
        let _ = raster::save(&image, &format!("{}/{:06}.png", path, i + 1));
    }
}
