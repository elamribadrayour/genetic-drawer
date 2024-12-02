use crate::genetic_algorithm::population::Population;
use crate::genetic_algorithm::selections::{Config, Selection};

use rand::RngCore;

pub struct Elite {
    pub rate: f64,
}

impl Elite {
    pub fn new(config: &Config) -> Self {
        Elite { rate: config.rate }
    }
}

impl Selection for Elite {
    fn select(
        &self,
        _rng: &mut dyn RngCore,
        population: &Population,
        fitnesses: &[f64],
    ) -> Population {
        let individual_size = population.individuals[0].len();
        let selection_size = (population.len() as f64 * self.rate).ceil() as usize;
        let mut selected = Population::empty(selection_size, individual_size);

        let mut indices: Vec<usize> = (0..population.len()).collect();
        indices.sort_by(|&a, &b| fitnesses[b].partial_cmp(&fitnesses[a]).unwrap());

        (0..selection_size).for_each(|i| {
            selected.set(i, population.get(indices[i]).clone());
        });

        selected
    }
}
