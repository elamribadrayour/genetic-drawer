use crate::genetic_algorithm::population::Point;

use rand::{Rng, RngCore};

#[derive(Clone)]
pub struct Triangle {
    center: Point,
    base: f64,
    height: f64,
    max_side: f64,
}

impl Triangle {
    pub fn new(rng: &mut dyn RngCore, max_area: &Option<f64>) -> Self {
        let max_side = (max_area.unwrap_or(1.0) / 2.0).sqrt();
        Triangle {
            center: Point::new(rng),
            base: rng.gen_range(0.0..=1.0),
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
        self.base += delta[2];
        self.base = self.base.clamp(0.0, self.max_side);
        self.height += delta[3];
        self.height = self.height.clamp(0.0, self.max_side);
    }

    pub fn points(&self, width: u32, height: u32) -> Vec<imageproc::point::Point<i32>> {
        let half_base = self.base / 2.0;
        let width = width as f64;
        let height = height as f64;
        vec![
            imageproc::point::Point::new(
                (self.center.x * width) as i32,
                ((self.center.y - self.height) * height) as i32,
            ),
            imageproc::point::Point::new(
                ((self.center.x - half_base) * width) as i32,
                (self.center.y * height) as i32,
            ),
            imageproc::point::Point::new(
                ((self.center.x + half_base) * width) as i32,
                (self.center.y * height) as i32,
            ),
        ]
    }
}
