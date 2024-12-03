use crate::genetic_algorithm::population::{Config, Individual};

use rand::RngCore;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct Population {
    pub individuals: Vec<Individual>,
}

impl Population {
    pub fn new(config: &Config, rng: &mut dyn RngCore) -> Self {
        Self {
            individuals: (0..config.population_size)
                .map(|i| {
                    Individual::new(
                        rng,
                        i,
                        config.individual_size,
                        &config.shapes,
                        &config.max_area,
                        &config.color_type,
                    )
                })
                .collect(),
        }
    }

    pub fn empty(size: usize, individual_size: usize) -> Self {
        Self {
            individuals: (0..size)
                .map(|i| Individual::empty(i, individual_size))
                .collect(),
        }
    }

    pub fn par_iter(&self) -> impl ParallelIterator<Item = &Individual> {
        self.individuals.par_iter()
    }

    pub fn len(&self) -> usize {
        self.individuals.len()
    }

    pub fn get(&self, id: usize) -> &Individual {
        self.individuals.get(id).unwrap()
    }

    pub fn get_mut(&mut self, id: usize) -> &mut Individual {
        self.individuals.get_mut(id).unwrap()
    }

    pub fn set(&mut self, id: usize, individual: Individual) {
        self.individuals[id] = individual;
    }
}
