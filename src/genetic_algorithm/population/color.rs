use image::Rgba;
use rand::{Rng, RngCore};

#[derive(Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        Color {
            r: rng.gen_range(0.0..=1.0),
            g: rng.gen_range(0.0..=1.0),
            b: rng.gen_range(0.0..=1.0),
            a: rng.gen_range(0.0..=1.0),
        }
    }

    pub fn empty() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }

    pub fn update(&mut self, delta: &[f64]) {
        if delta.len() != 4 {
            panic!(
                "invalid color delta length {} while expected 4",
                delta.len()
            );
        }
        self.r += delta[0];
        self.g += delta[1];
        self.b += delta[2];
        self.a += delta[3];
    }

    pub fn rgba(&self) -> Rgba<u8> {
        Rgba::from([
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        ])
    }
}
