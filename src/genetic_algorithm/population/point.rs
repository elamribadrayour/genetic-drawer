use rand::{Rng, RngCore};

#[derive(Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        Point {
            x: rng.gen_range(0.0..=1.0),
            y: rng.gen_range(0.0..=1.0),
        }
    }

    pub fn update(&mut self, delta: &[f64]) {
        if delta.len() != 2 {
            panic!("expected data size is 2 --> found {}", delta.len());
        }
        self.x += delta[0];
        self.y += delta[1];
    }
}
