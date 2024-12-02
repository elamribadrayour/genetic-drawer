use crate::genetic_algorithm::population::Gene;

use image::RgbImage;

#[derive(Clone)]
pub struct Individual {
    pub id: usize,
    pub genes: Vec<Gene>,
}

impl Individual {
    pub fn new(
        rng: &mut dyn rand::RngCore,
        id: usize,
        individual_size: usize,
        shapes: &[String],
        color_type: &Option<String>,
    ) -> Self {
        Self {
            id,
            genes: (0..individual_size)
                .map(|_| Gene::new(rng, shapes, color_type))
                .collect(),
        }
    }

    pub fn empty(id: usize, individual_size: usize) -> Self {
        Self {
            id,
            genes: (0..individual_size).map(|_| Gene::empty()).collect(),
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
        format!("./.cache/frames/{}.png", self.id)
    }

    pub fn draw(&self, dimensions: &(u32, u32)) {
        let (width, height) = dimensions;
        let mut image = RgbImage::new(*width, *height);
        self.genes.iter().for_each(|gene| {
            gene.draw(&mut image);
        });
        image.save(&self.path()).unwrap();
    }

    pub fn read(&self) -> RgbImage {
        image::open(self.path()).unwrap().to_rgb8()
    }
}
