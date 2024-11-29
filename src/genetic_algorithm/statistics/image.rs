// Image statistics

use serde_json::{json, Value};

pub struct Image;

impl Image {
    pub fn get_image_stats(id: usize) -> Value {
        // Compute image statistics
        // Return a JSON object with the statistics
        let file_path = format!(".cache/frames/{}.png", id);
        let image = image::open(file_path).unwrap();

        // Calculate mean and max pixel values
        let mut total_pixel_value = 0u64;
        let mut max_pixel_value = 0u8;
        let pixel_count = (image.width() * image.height()) as u64;

        let image_buffer = image.to_luma8(); // Convert to a grayscale image buffer

        for pixel in image_buffer.pixels() {
            let pixel_value = pixel[0];
            total_pixel_value += pixel_value as u64;
            if pixel_value > max_pixel_value {
                max_pixel_value = pixel_value;
            }
        }

        let mean_pixel_value = total_pixel_value / pixel_count;

        json!({
            "mean_pixel_value": mean_pixel_value,
            "max_pixel_value": max_pixel_value,
        })
    }

    pub fn get(size: usize) -> Value {
        let stats: Vec<Value> = (0..size).map(Self::get_image_stats).collect();
        json!(stats)
    }
}
