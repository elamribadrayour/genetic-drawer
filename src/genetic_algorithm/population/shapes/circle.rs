use crate::genetic_algorithm::population::Point;
use rand::{Rng, RngCore};

#[derive(Clone)]
pub struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        Circle {
            center: Point::new(rng),
            radius: rng.gen_range(0.0..=1.0),
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
        self.radius += delta[2];
    }

    pub fn points(&self, width: u32, height: u32) -> Vec<imageproc::point::Point<i32>> {
        let num_points = 10;
        let radius = self.radius;
        let width = width as f64;
        let height = height as f64;
        let mut points = Vec::with_capacity(num_points);

        (0..num_points).for_each(|i| {
            let theta = 2.0 * std::f64::consts::PI * (i as f64) / (num_points as f64);
            let x = self.center.x + radius * theta.cos();
            let y = self.center.y + radius * theta.sin();
            points.push(imageproc::point::Point::new(
                (x * width) as i32,
                (y * height) as i32,
            ));
        });

        points
    }
}
