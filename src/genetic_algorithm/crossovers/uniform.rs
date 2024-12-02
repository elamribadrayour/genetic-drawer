use crate::genetic_algorithm::crossovers::Crossover;
use crate::genetic_algorithm::population::{Individual, Population};

use rand::{Rng, RngCore};

pub struct Uniform {
    pub rate: f64,
}

impl Uniform {
    pub fn new(rate: Option<f64>) -> Self {
        if rate.is_none() {
            panic!("rate is required for uniform crossover");
        }
        Self {
            rate: rate.unwrap(),
        }
    }
}

impl Crossover for Uniform {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        population: &Population,
        population_size: usize,
    ) -> Population {
        let individual_size = population.individuals[0].len();
        let mut children = Population::empty(population_size, individual_size);

        (0..population_size).for_each(|i| {
            let i_1 = rng.gen_range(0..population.len());
            let i_2 = rng.gen_range(0..population.len());

            let parent_1 = population.get(i_1);
            let parent_2 = population.get(i_2);

            let mut individual_1 = Individual::empty(i, individual_size);
            let mut individual_2 = Individual::empty(i + 1, individual_size);

            (0..individual_size).for_each(|j| {
                if rng.gen_bool(self.rate) {
                    individual_1.set(j, parent_1.get(j));
                    individual_2.set(j, parent_2.get(j));
                } else {
                    individual_1.set(j, parent_2.get(j));
                    individual_2.set(j, parent_1.get(j));
                }
            });

            children.set(i, individual_1);
            if i + 1 < population_size {
                children.set(i + 1, individual_2);
            }
        });
        children
    }
}
