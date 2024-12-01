use crate::genetic_algorithm::population::Population;
use crate::genetic_algorithm::selections::{Config, Selection};

use rand::{Rng, RngCore};

pub struct TournamentSelection {
    pub rate: f64,
}

impl TournamentSelection {
    pub fn new(config: &Config) -> Self {
        TournamentSelection { rate: config.rate }
    }
}

impl Selection for TournamentSelection {
    fn select(
        &self,
        rng: &mut dyn RngCore,
        population: &Population,
        fitnesses: &[f64],
    ) -> Population {
        let individual_size = population.individuals[0].len();
        let selection_size = (population.len() as f64 * self.rate).ceil() as usize;
        let mut selected = Population::empty(selection_size, individual_size);

        for i in 0..selection_size {
            let tournament: Vec<usize> = (0..selection_size)
                .map(|_| rng.gen_range(0..population.len()))
                .collect();

            let best = tournament
                .iter()
                .max_by(|a, b| fitnesses[**a].partial_cmp(&fitnesses[**b]).unwrap())
                .unwrap();
            selected.set(i, population.get(*best).clone());
        }

        selected
    }
}
