use crate::genetic_algorithm::mutations::{Config, Mutation};
use crate::genetic_algorithm::population::{Color, Gene, Individual, Population, Shape};

use rand::{Rng, RngCore};
use rand_distr::{Distribution, Normal};

pub struct GaussianMutation {
    pub rate: f64,
    pub distribution: Normal<f64>,
}

impl GaussianMutation {
    pub fn new(config: &Config) -> Self {
        if config.mu.is_none() || config.sigma.is_none() {
            panic!("Mu and sigma must be provided");
        }
        Self {
            rate: config.rate,
            distribution: Normal::new(config.mu.unwrap(), config.sigma.unwrap()).unwrap(),
        }
    }
}

impl Mutation for GaussianMutation {
    fn mutate_shape(&self, rng: &mut dyn RngCore, shape: &mut Shape) {
        let delta: Vec<f64> = (0..shape.len())
            .map(|_| self.distribution.sample(rng))
            .collect();
        shape.update(&delta);
    }

    fn mutate_color(&self, rng: &mut dyn RngCore, color: &mut Color) {
        let delta: Vec<f64> = (0..4).map(|_| self.distribution.sample(rng)).collect();
        color.update(&delta);
    }

    fn mutate_gene(&self, rng: &mut dyn RngCore, gene: &mut Gene) {
        self.mutate_shape(rng, &mut gene.shape);
        self.mutate_color(rng, &mut gene.color);
    }

    fn mutate_individual(&self, rng: &mut dyn RngCore, individual: &mut Individual) {
        individual.genes.iter_mut().for_each(|gene| {
            self.mutate_gene(rng, gene);
        });
    }

    fn mutate(&self, rng: &mut dyn RngCore, population: &mut Population) -> usize {
        let mut mutated = 0;
        for i in 0..population.len() {
            let individual = population.get_mut(i);
            if !rng.gen_bool(self.rate) {
                continue;
            }
            self.mutate_individual(rng, individual);
            mutated += 1;
        }
        mutated
    }
}
