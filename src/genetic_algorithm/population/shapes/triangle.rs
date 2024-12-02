use rand::{Rng, RngCore};

#[derive(Clone)]
pub struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Triangle {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        Triangle {
            a: rng.gen_range(0.0..=1.0),
            b: rng.gen_range(0.0..=1.0),
            c: rng.gen_range(0.0..=1.0),
        }
    }

    pub fn len(&self) -> usize {
        3
    }

    pub fn update(&mut self, delta: &[f64]) {
        if delta.len() != 3 {
            panic!("expected data size is 3 --> found {}", delta.len());
        }
        self.a += delta[0];
        self.b += delta[1];
        self.c += delta[2];
    }

    pub fn points(&self, width: u32, height: u32) -> Vec<imageproc::point::Point<i32>> {
        let width = width as f64;
        let height = height as f64;

        let x1 = self.a * width;
        let y1 = self.b * height;

        let x2 = self.b * width;
        let y2 = self.c * height;

        let x3 = self.c * width;
        let y3 = self.a * height;

        vec![
            imageproc::point::Point::new(x1 as i32, y1 as i32),
            imageproc::point::Point::new(x2 as i32, y2 as i32),
            imageproc::point::Point::new(x3 as i32, y3 as i32),
        ]
    }
}
