use crate::genetic_algorithm::fitnesses::Fitness;

use image::RgbImage;

pub struct Mae;

impl Fitness for Mae {
    fn calculate(&self, source: &RgbImage, result: &RgbImage) -> f64 {
        let mut error = 0.0;
        let nb_pixels = (source.width() * source.height()) as f64;
        if nb_pixels == 0.0 {
            panic!("nb_pixels is 0");
        }

        for (pixel, target_pixel) in source.pixels().zip(result.pixels()) {
            error += (pixel[0] as f64 - target_pixel[0] as f64).abs()
                + (pixel[1] as f64 - target_pixel[1] as f64).abs()
                + (pixel[2] as f64 - target_pixel[2] as f64).abs();
        }

        let max_error = 255.0 * 3.0; // Maximum possible absolute error per pixel
        let mae = error / (nb_pixels * max_error);

        // Normalize and invert the MAE for fitness calculation.
        // This ensures that lower MAE values (better matches) result in higher fitness scores.
        1.0 / (1.0 + mae)
    }
}
