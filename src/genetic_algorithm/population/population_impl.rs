use crate::genetic_algorithm::Individual;

use rand::RngCore;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct Population {
    pub individuals: Vec<Individual>,
}

impl Population {
    pub fn new(
        rng: &mut dyn RngCore,
        size: usize,
        nb_polygons: usize,
        polygon_size: usize,
    ) -> Self {
        Self {
            individuals: (0..size)
                .map(|i| Individual::new(rng, i, nb_polygons, polygon_size))
                .collect(),
        }
    }

    pub fn empty(size: usize, nb_genes: usize, polygon_size: usize) -> Self {
        Self {
            individuals: (0..size)
                .map(|i| Individual::empty(i, nb_genes, polygon_size))
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
