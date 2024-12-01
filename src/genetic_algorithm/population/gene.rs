use image::RgbaImage;
use imageproc::drawing;
use rand::RngCore;

use crate::genetic_algorithm::population::{Color, Shape};

#[derive(Clone)]
pub struct Gene {
    pub shape: Shape,
    pub color: Color,
}

impl Gene {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        Self {
            shape: Shape::new(rng),
            color: Color::new(rng),
        }
    }

    pub fn empty() -> Self {
        Self {
            shape: Shape::empty(),
            color: Color::empty(),
        }
    }

    pub fn draw(&self, image: &mut RgbaImage) {
        let color = self.color.rgba();
        let points = self.shape.points(image.width(), image.height());
        if points.first() == points.last() {
            return;
        }
        drawing::draw_polygon_mut(image, &points, color);
    }
}
