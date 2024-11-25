use crate::genetic_algorithm::Fitness;

use image::RgbaImage;

pub struct Mse;

impl Fitness for Mse {
    fn calculate(&self, source: &RgbaImage, result: &RgbaImage) -> f32 {
        let mut error = 0.0;
        let nb_pixels = (source.width() * source.height()) as f32;
        if nb_pixels == 0.0 {
            panic!("nb_pixels is 0");
        }

        for (pixel, target_pixel) in source.pixels().zip(result.pixels()) {
            error += (pixel[0] as f32 - target_pixel[0] as f32).powi(2)
                + (pixel[1] as f32 - target_pixel[1] as f32).powi(2)
                + (pixel[2] as f32 - target_pixel[2] as f32).powi(2)
                + (pixel[3] as f32 - target_pixel[3] as f32).powi(2);
        }

        let mse = error / nb_pixels;
        1.0 / (1.0 + mse)
    }
}
