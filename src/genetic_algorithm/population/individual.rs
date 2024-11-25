use crate::genetic_algorithm::Gene;

use image::RgbaImage;

#[derive(Clone)]
pub struct Individual {
    pub id: usize,
    pub genes: Vec<Gene>,
}

impl Individual {
    pub fn new(rng: &mut dyn rand::RngCore, id: usize, size: usize, polygon_size: usize) -> Self {
        Self {
            id,
            genes: (0..size).map(|_| Gene::new(rng, polygon_size)).collect(),
        }
    }

    pub fn empty(id: usize, size: usize, polygon_size: usize) -> Self {
        Self {
            id,
            genes: (0..size).map(|_| Gene::empty(polygon_size)).collect(),
        }
    }
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn set(&mut self, index: usize, gene: &Gene) {
        self.genes[index] = gene.clone();
    }

    pub fn get(&self, index: usize) -> &Gene {
        self.genes.get(index).unwrap()
    }

    pub fn path(&self) -> String {
        format!("./.cache/frame-{}.png", self.id)
    }

    pub fn draw(&self, dimensions: &(u32, u32)) {
        let (width, height) = dimensions;
        let mut image = RgbaImage::new(*width, *height);
        self.genes.iter().for_each(|gene| {
            gene.draw(&mut image);
        });
        image.save(&self.path()).unwrap();
    }

    pub fn read(&self) -> RgbaImage {
        image::open(self.path()).unwrap().to_rgba8()
    }
}
