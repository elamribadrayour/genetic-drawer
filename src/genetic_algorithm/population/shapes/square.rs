use crate::genetic_algorithm::population::Point;

use rand::{Rng, RngCore};

#[derive(Clone)]
pub struct Square {
    center: Point,
    side: f64,
    max_side: f64,
}

impl Square {
    pub fn new(rng: &mut dyn RngCore, max_area: &Option<f64>) -> Self {
        let max_side = (max_area.unwrap_or(1.0) / 4.0).sqrt();
        Square {
            center: Point::new(rng),
            side: rng.gen_range(0.0..=max_side),
            max_side,
        }
    }

    pub fn len(&self) -> usize {
        3
    }

    pub fn update(&mut self, delta: &[f64]) {
        if delta.len() != 3 {
            panic!("expected data size is 3 --> found {}", delta.len());
        }
        self.center.update(&delta[0..2]);
        self.side += delta[2];
        self.side = self.side.clamp(0.0, self.max_side);
    }

    pub fn points(&self, width: u32, height: u32) -> Vec<imageproc::point::Point<i32>> {
        let half = self.side / 2.0;
        let width = width as f64;
        let height = height as f64;
        vec![
            imageproc::point::Point::new(
                ((self.center.x - half) * width) as i32,
                ((self.center.y - half) * height) as i32,
            ),
            imageproc::point::Point::new(
                ((self.center.x + half) * width) as i32,
                ((self.center.y - half) * height) as i32,
            ),
            imageproc::point::Point::new(
                ((self.center.x + half) * width) as i32,
                ((self.center.y + half) * height) as i32,
            ),
            imageproc::point::Point::new(
                ((self.center.x - half) * width) as i32,
                ((self.center.y + half) * height) as i32,
            ),
        ]
    }
}
