use crate::genetic_algorithm::population::Point;

use rand::{Rng, RngCore};

#[derive(Clone)]
pub struct Rectangle {
    center: Point,
    width: f64,
    height: f64,
    max_side: f64,
}

impl Rectangle {
    pub fn new(rng: &mut dyn RngCore, max_area: &Option<f64>) -> Self {
        let max_side = (max_area.unwrap_or(1.0) / 2.0).sqrt();
        Rectangle {
            center: Point::new(rng),
            width: rng.gen_range(0.0..=1.0),
            height: rng.gen_range(0.0..=max_side),
            max_side,
        }
    }

    pub fn len(&self) -> usize {
        4
    }

    pub fn update(&mut self, delta: &[f64]) {
        if delta.len() != 4 {
            panic!("expected data size is 4 --> found {}", delta.len());
        }
        self.center.update(&delta[0..2]);
        self.width += delta[2];
        self.width = self.width.clamp(0.0, self.max_side);
        self.height += delta[3];
        self.height = self.height.clamp(0.0, self.max_side);
    }

    pub fn points(&self, width: u32, height: u32) -> Vec<imageproc::point::Point<i32>> {
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        let width = width as f64;
        let height = height as f64;
        vec![
            imageproc::point::Point::new(
                ((self.center.x - half_width) * width) as i32,
                ((self.center.y - half_height) * height) as i32,
            ),
            imageproc::point::Point::new(
                ((self.center.x + half_width) * width) as i32,
                ((self.center.y - half_height) * height) as i32,
            ),
            imageproc::point::Point::new(
                ((self.center.x + half_width) * width) as i32,
                ((self.center.y + half_height) * height) as i32,
            ),
            imageproc::point::Point::new(
                ((self.center.x - half_width) * width) as i32,
                ((self.center.y + half_height) * height) as i32,
            ),
        ]
    }
}
