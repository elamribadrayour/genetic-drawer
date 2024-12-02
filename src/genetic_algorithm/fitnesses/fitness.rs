use image::RgbImage;

pub trait Fitness {
    fn calculate(&self, source: &RgbImage, result: &RgbImage) -> f64;
}
