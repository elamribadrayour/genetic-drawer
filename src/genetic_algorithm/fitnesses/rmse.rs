use crate::genetic_algorithm::{Fitness, Mse};

use image::RgbaImage;

pub struct Rmse;

impl Fitness for Rmse {
    fn calculate(&self, source: &RgbaImage, result: &RgbaImage) -> f32 {
        let mse = Mse.calculate(source, result);
        mse.sqrt()
    }
}
