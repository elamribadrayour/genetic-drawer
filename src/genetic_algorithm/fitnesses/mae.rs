use crate::genetic_algorithm::Fitness;

use image::RgbaImage;

pub struct Mae;

impl Fitness for Mae {
    fn calculate(&self, source: &RgbaImage, result: &RgbaImage) -> f64 {
        let mut error = 0.0;
        let nb_pixels = (source.width() * source.height()) as f64;
        if nb_pixels == 0.0 {
            panic!("nb_pixels is 0");
        }

        for (pixel, target_pixel) in source.pixels().zip(result.pixels()) {
            error += (pixel[0] as f64 - target_pixel[0] as f64).abs()
                + (pixel[1] as f64 - target_pixel[1] as f64).abs()
                + (pixel[2] as f64 - target_pixel[2] as f64).abs()
                + (pixel[3] as f64 - target_pixel[3] as f64).abs();
        }

        let mae = error / nb_pixels;
        1.0 / (1.0 + mae)
    }
}
