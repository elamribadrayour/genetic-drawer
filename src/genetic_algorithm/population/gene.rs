use image::{Rgba, RgbaImage};
use imageproc::drawing;
use imageproc::point::Point;
use rand::{Rng, RngCore};

#[derive(Clone)]
pub struct Gene {
    pub points: Vec<Point<f32>>,
    pub color: [f32; 4],
}

impl Gene {
    pub fn new(rng: &mut dyn RngCore, polygon_size: usize) -> Self {
        // A gene is represented by an array of 10 floats, each between 0 and 1.
        // The first 6 elements represent the points of a polygon.
        // The next 3 elements represent the color of the polygon (RGB).
        // The last element represents the alpha (transparency) value.
        Self {
            points: (0..polygon_size)
                .map(|_| Point::new(rng.gen_range(0.0..=1.0), rng.gen_range(0.0..=1.0)))
                .collect(),
            color: [rng.gen_range(0.0..=1.0); 4],
        }
    }

    pub fn empty(polygon_size: usize) -> Self {
        Self {
            points: vec![Point::new(0.0, 0.0); polygon_size],
            color: [0.0; 4],
        }
    }

    pub fn set_color(&mut self, mult: f32, i: usize) {
        let new_color = mult * self.color[i];
        if new_color < 0.0 {
            self.color[i] = 0.0;
        } else if new_color > 1.0 {
            self.color[i] = 1.0;
        } else {
            self.color[i] = new_color;
        }
    }

    pub fn set_point(&mut self, mult: (f32, f32), i: usize) {
        let new_point = Point::new(mult.0 * self.points[i].x, mult.1 * self.points[i].y);
        if new_point.x < 0.0 {
            self.points[i].x = 0.0;
        } else if new_point.x > 1.0 {
            self.points[i].x = 1.0;
        } else {
            self.points[i].x = new_point.x;
        }

        if new_point.y < 0.0 {
            self.points[i].y = 0.0;
        } else if new_point.y > 1.0 {
            self.points[i].y = 1.0;
        } else {
            self.points[i].y = new_point.y;
        }
    }

    pub fn points(&self, width: u32, height: u32) -> Vec<Point<i32>> {
        self.points
            .iter()
            .map(|p| Point::new((p.x * width as f32) as i32, (p.y * height as f32) as i32))
            .collect()
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
        let points: Vec<Point<i32>> = self.points(image.width(), image.height());
        if points.first() == points.last() {
            return;
        }
        drawing::draw_polygon_mut(image, &points, color);
    }
}
