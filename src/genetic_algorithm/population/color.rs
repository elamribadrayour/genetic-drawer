use image::Rgb;
use rand::{Rng, RngCore};

#[derive(Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub color_type: String,
}

impl Color {
    pub fn new(rng: &mut dyn RngCore, color_type: String) -> Self {
        if color_type != "rgb" && color_type != "grayscale" {
            panic!("invalid color type {}", color_type);
        }

        let r = rng.gen_range(0.0..=1.0);
        let g = rng.gen_range(0.0..=1.0);
        let b = rng.gen_range(0.0..=1.0);
        Color {
            r,
            g,
            b,
            color_type,
        }
    }

    pub fn empty() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            color_type: "rgb".to_string(),
        }
    }

    pub fn update(&mut self, delta: &[f64]) {
        if delta.len() != 3 {
            panic!(
                "invalid color delta length {} while expected 3",
                delta.len()
            );
        }
        self.r += delta[0];
        self.g += delta[1];
        self.b += delta[2];
    }

    pub fn rgb(&self) -> Rgb<u8> {
        match self.color_type.as_str() {
            "rgb" => Rgb::from([
                (self.r * 255.0) as u8,
                (self.g * 255.0) as u8,
                (self.b * 255.0) as u8,
            ]),
            "grayscale" => {
                let luminance = ((0.299 * self.r + 0.587 * self.g + 0.114 * self.b) * 255.0) as u8;
                Rgb::from([luminance, luminance, luminance])
            }
            _ => panic!("invalid color type {}", self.color_type),
        }
    }
}
