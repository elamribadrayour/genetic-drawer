use image::{Rgba, RgbaImage};
use imageproc::drawing;
use rand::{Rng, RngCore};

use crate::genetic_algorithm::Point;

#[derive(Clone)]
pub struct Gene {
    pub values: Vec<Point>,
    pub color: [f64; 4],
}

impl Gene {
    pub fn new(rng: &mut dyn RngCore, polygon_size: usize) -> Self {
        // A gene is represented by an array of 10 floats, each between 0 and 1.
        // The first 6 elements represent the points of a polygon.
        // The next 3 elements represent the color of the polygon (RGB).
        // The last element represents the alpha (transparency) value.
        Self {
            values: (0..polygon_size)
                .map(|_| Point {
                    x: rng.gen_range(0.0..=1.0),
                    y: rng.gen_range(0.0..=1.0),
                })
                .collect(),
            color: [rng.gen_range(0.0..=1.0); 4],
        }
    }

    pub fn empty(polygon_size: usize) -> Self {
        Self {
            values: vec![Point { x: 0.0, y: 0.0 }; polygon_size],
            color: [0.0; 4],
        }
    }

    pub fn set_color(&mut self, delta: &[f64]) {
        (0..4).for_each(|i| {
            self.color[i] += delta[i];
            if self.color[i] < f64::EPSILON {
                self.color[i] = 0.0;
            } else if (self.color[i] - 1.0).abs() < f64::EPSILON {
                self.color[i] = 1.0;
            }
        });
    }

    pub fn set_point(&mut self, delta: (f64, f64), i: usize) {
        let new_point = Point {
            x: self.values[i].x + delta.0,
            y: self.values[i].y + delta.1,
        };
        if new_point.x < 0.0 {
            self.values[i].x = 0.0;
        } else if new_point.x > 1.0 {
            self.values[i].x = 1.0;
        } else {
            self.values[i].x = new_point.x;
        }

        if new_point.y < 0.0 {
            self.values[i].y = 0.0;
        } else if new_point.y > 1.0 {
            self.values[i].y = 1.0;
        } else {
            self.values[i].y = new_point.y;
        }
    }

    pub fn color(&self) -> Rgba<u8> {
        Rgba::from([
            (self.color[0] * 255.0) as u8,
            (self.color[1] * 255.0) as u8,
            (self.color[2] * 255.0) as u8,
            (self.color[3] * 255.0) as u8,
        ])
    }

    pub fn draw(&self, image: &mut RgbaImage) {
        let color = self.color();
        let points: Vec<imageproc::point::Point<i32>> = self
            .values
            .iter()
            .map(|p| {
                imageproc::point::Point::new(
                    (p.x * image.width() as f64) as i32,
                    (p.y * image.height() as f64) as i32,
                )
            })
            .collect();
        if points.first() == points.last() {
            return;
        }
        drawing::draw_polygon_mut(image, &points, color);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}
