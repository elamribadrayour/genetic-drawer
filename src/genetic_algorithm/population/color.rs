use image::Rgb;
use rand::{Rng, RngCore};

#[derive(Clone)]
pub struct Color {
    pub color: [f64; 3],
    pub color_type: String,
}

impl Color {
    pub fn new(rng: &mut dyn RngCore, color_type: String) -> Self {
        if color_type != "rgb" && color_type != "grayscale" {
            panic!("invalid color type {}", color_type);
        }

        let color: [f64; 3] = [rng.gen_range(0.0..=1.0); 3];
        Color { color, color_type }
    }

    pub fn empty() -> Self {
        Color {
            color: [0.0; 3],
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
        self.color.iter_mut().zip(delta.iter()).for_each(|(c, d)| {
            *c += d;
            *c = c.clamp(0.0, 1.0);
        });
    }

    pub fn rgb(&self) -> Rgb<u8> {
        match self.color_type.as_str() {
            "rgb" => Rgb::from([
                (self.color[0] * 255.0) as u8,
                (self.color[1] * 255.0) as u8,
                (self.color[2] * 255.0) as u8,
            ]),
            "grayscale" => {
                let multiplier = [0.299, 0.587, 0.114];
                let luminance = (self
                    .color
                    .iter()
                    .zip(multiplier.iter())
                    .map(|(c, m)| c * m)
                    .sum::<f64>()
                    * 255.0) as u8;
                Rgb::from([luminance, luminance, luminance])
            }
            _ => panic!("invalid color type {}", self.color_type),
        }
    }
}
