use crate::genetic_algorithm::{Gene, Individual, Mutation, Population};

use rand::{Rng, RngCore};
use rand_distr::Normal;

pub struct GaussianMutation {
    pub mu: f64,
    pub sigma: f64,
}

impl Mutation for GaussianMutation {
    fn mutate_gene(&self, rng: &mut dyn RngCore, gene: &mut Gene) {
        // Helper function to generate Gaussian random numbers
        let gaussian_random = |rng: &mut dyn RngCore, mu: f64, sigma: f64| -> f64 {
            // Assuming you have a Gaussian distribution generator
            // Replace with appropriate Gaussian random number generation
            mu + sigma * rng.sample(Normal::new(0.0, 1.0).unwrap())
        };

        // Mutate points
        (0..gene.values.len()).for_each(|j| {
            let x = gaussian_random(rng, self.mu, self.sigma);
            let y = gaussian_random(rng, self.mu, self.sigma);
            gene.set_point((x, y), j);
        });

        // Mutate colors
        let r = gaussian_random(rng, self.mu, self.sigma);
        let g = gaussian_random(rng, self.mu, self.sigma);
        let b = gaussian_random(rng, self.mu, self.sigma);
        let a = gaussian_random(rng, self.mu, self.sigma);
        gene.set_color(&[r, g, b, a]);
    }

    fn mutate_individual(&self, rng: &mut dyn RngCore, individual: &mut Individual) {
        individual.genes.iter_mut().for_each(|gene| {
            self.mutate_gene(rng, gene);
        });
    }

    fn mutate(
        &self,
        rng: &mut dyn RngCore,
        population: &mut Population,
        mutation_rate: f64,
    ) -> usize {
        let mut mutated = 0;
        for i in 0..population.len() {
            let individual = population.get_mut(i);
            if !rng.gen_bool(mutation_rate) {
                continue;
            }
            self.mutate_individual(rng, individual);
            mutated += 1;
        }
        mutated
    }
}
