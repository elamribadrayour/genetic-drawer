use crate::genetic_algorithm::fitnesses::{Fitness, Mse};

use image::RgbaImage;

pub struct Rmse;

impl Fitness for Rmse {
    fn calculate(&self, source: &RgbaImage, result: &RgbaImage) -> f64 {
        let mse = Mse.calculate(source, result);
        mse.sqrt()
    }
}
