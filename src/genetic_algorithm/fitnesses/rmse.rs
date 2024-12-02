use crate::genetic_algorithm::fitnesses::{Fitness, Mse};

use image::RgbImage;

pub struct Rmse;

impl Fitness for Rmse {
    fn calculate(&self, source: &RgbImage, result: &RgbImage) -> f64 {
        let mse = Mse.calculate(source, result);
        mse.sqrt()
    }
}
