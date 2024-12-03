use crate::genetic_algorithm::fitnesses::Fitness;

use image::RgbImage;

pub struct Mse;

impl Fitness for Mse {
    fn calculate(&self, source: &RgbImage, result: &RgbImage) -> f64 {
        let mut error = 0.0;
        let nb_pixels = (source.width() * source.height()) as f64;
        if nb_pixels == 0.0 {
            panic!("nb_pixels is 0");
        }

        for (pixel, target_pixel) in source.pixels().zip(result.pixels()) {
            error += (pixel[0] as f64 - target_pixel[0] as f64).powi(2)
                + (pixel[1] as f64 - target_pixel[1] as f64).powi(2)
                + (pixel[2] as f64 - target_pixel[2] as f64).powi(2);
        }

        let max_error = 255.0 * 255.0 * 3.0;
        let mse = error / (nb_pixels * max_error);

        // Normalize and invert the MSE for fitness calculation.
        // This ensures that lower MSE values (better matches) result in higher fitness scores.
        1.0 / (1.0 + mse)
    }
}
