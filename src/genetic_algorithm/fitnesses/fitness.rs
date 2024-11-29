use image::RgbaImage;

pub trait Fitness {
    fn calculate(&self, source: &RgbaImage, result: &RgbaImage) -> f64;
}
