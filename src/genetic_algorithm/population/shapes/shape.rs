use rand::{Rng, RngCore};

use crate::genetic_algorithm::population::shapes::{Circle, Rectangle, Square};

#[derive(Clone)]
pub enum Shape {
    Empty,
    Square(Square),
    Circle(Circle),
    Rectangle(Rectangle),
}

impl Shape {
    pub fn new(rng: &mut dyn RngCore) -> Self {
        let name = rng.gen_range(0..=2);
        match name {
            0 => Shape::Square(Square::new(rng)),
            1 => Shape::Circle(Circle::new(rng)),
            2 => Shape::Rectangle(Rectangle::new(rng)),
            _ => panic!("invalid shape name {}", name),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Shape::Square(square) => square.len(),
            Shape::Circle(circle) => circle.len(),
            Shape::Rectangle(rectangle) => rectangle.len(),
            Shape::Empty => 0,
        }
    }

    pub fn empty() -> Self {
        Shape::Empty
    }

    pub fn update(&mut self, delta: &[f64]) {
        match self {
            Shape::Square(square) => square.update(delta),
            Shape::Circle(circle) => circle.update(delta),
            Shape::Rectangle(rectangle) => rectangle.update(delta),
            Shape::Empty => (),
        }
    }

    pub fn points(&self, width: u32, height: u32) -> Vec<imageproc::point::Point<i32>> {
        match self {
            Shape::Square(square) => square.points(width, height),
            Shape::Circle(circle) => circle.points(width, height),
            Shape::Rectangle(rectangle) => rectangle.points(width, height),
            Shape::Empty => vec![],
        }
    }
}
