use image::RgbImage;
use imageproc::drawing;
use rand::RngCore;

use crate::genetic_algorithm::population::{Color, Shape};

#[derive(Clone)]
pub struct Gene {
    pub shape: Shape,
    pub color: Color,
}

impl Gene {
    pub fn new(rng: &mut dyn RngCore, shapes: &[String], color_type: &Option<String>) -> Self {
        let color_type = color_type.clone().unwrap_or("rgb".to_string());
        Self {
            shape: Shape::new(rng, shapes),
            color: Color::new(rng, color_type),
        }
    }

    pub fn empty() -> Self {
        Self {
            shape: Shape::empty(),
            color: Color::empty(),
        }
    }

    pub fn draw(&self, image: &mut RgbImage) {
        let color = self.color.rgb();
        let points = self.shape.points(image.width(), image.height());
        if points.first() == points.last() {
            return;
        }
        drawing::draw_polygon_mut(image, &points, color);
    }
}
