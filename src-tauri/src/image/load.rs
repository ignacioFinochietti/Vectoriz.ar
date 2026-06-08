use image::{DynamicImage, ImageError};

pub fn load_from_bytes(data: &[u8]) -> Result<DynamicImage, ImageError> {
    image::load_from_memory(data)
}

pub fn apply_gaussian_blur(img: &DynamicImage, radius: f64) -> DynamicImage {
    if radius <= 0.0 {
        return img.clone();
    }
    DynamicImage::ImageRgba8(image::imageops::blur(img, radius as f32))
}
