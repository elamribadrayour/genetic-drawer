use crate::genetic_algorithm::{Mutation, Population};

use rand::{Rng, RngCore};

pub struct GaussianMutation {
    pub mu: f32,
    pub sigma: f32,
}

impl Mutation for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, population: &mut Population, mutation_rate: f32) {
        for i in 0..population.len() {
            let individual = population.get_mut(i);
            if !rng.gen_bool(mutation_rate as f64) {
                continue;
            }
            individual.genes.iter_mut().for_each(|gene| {
                (0..gene.points.len()).for_each(|j| {
                    gene.set_point(
                        (
                            self.mu + self.sigma * rng.gen::<f32>(),
                            self.mu + self.sigma * rng.gen::<f32>(),
                        ),
                        j,
                    );
                });
                (0..4).for_each(|j| {
                    gene.set_color(self.mu + self.sigma * rng.gen::<f32>(), j);
                });
            });
        }
    }
}
