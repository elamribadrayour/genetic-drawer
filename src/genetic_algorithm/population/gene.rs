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
    pub fn new(rng: &mut dyn RngCore, shapes: &[String]) -> Self {
        Self {
            shape: Shape::new(rng, shapes),
            color: Color::new(rng),
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
