use rand::{Rng, RngCore};

use crate::genetic_algorithm::population::shapes::{Circle, Rectangle, Square, Triangle};

#[derive(Clone)]
pub enum Shape {
    Empty,
    Square(Square),
    Circle(Circle),
    Rectangle(Rectangle),
    Triangle(Triangle),
}

impl Shape {
    pub fn new(rng: &mut dyn RngCore, shapes: &[String], max_area: &Option<f64>) -> Self {
        let name = rng.gen_range(0..shapes.len());
        match shapes[name].as_str() {
            "square" => Shape::Square(Square::new(rng, max_area)),
            "circle" => Shape::Circle(Circle::new(rng, max_area)),
            "rectangle" => Shape::Rectangle(Rectangle::new(rng, max_area)),
            "triangle" => Shape::Triangle(Triangle::new(rng, max_area)),
            _ => panic!("invalid shape name {}", name),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Shape::Square(square) => square.len(),
            Shape::Circle(circle) => circle.len(),
            Shape::Rectangle(rectangle) => rectangle.len(),
            Shape::Triangle(triangle) => triangle.len(),
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
            Shape::Triangle(triangle) => triangle.update(delta),
            Shape::Empty => (),
        }
    }

    pub fn points(&self, width: u32, height: u32) -> Vec<imageproc::point::Point<i32>> {
        match self {
            Shape::Square(square) => square.points(width, height),
            Shape::Circle(circle) => circle.points(width, height),
            Shape::Rectangle(rectangle) => rectangle.points(width, height),
            Shape::Triangle(triangle) => triangle.points(width, height),
            Shape::Empty => vec![],
        }
    }
}
