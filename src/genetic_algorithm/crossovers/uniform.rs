use crate::genetic_algorithm::crossovers::Crossover;
use crate::genetic_algorithm::population::{Individual, Population};

use rand::seq::SliceRandom;
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
        let mut parents = (0..population.len()).collect::<Vec<usize>>();

        (0..population_size).step_by(2).for_each(|i| {
            // Take two random parents
            let i_1 = parents
                .choose(rng)
                .cloned()
                .expect("Failed to select parent 1");
            let parent_1 = population.get(i_1);
            parents.retain(|&x| x != i_1);

            let i_2 = parents
                .choose(rng)
                .cloned()
                .expect("Failed to select parent 2");
            let parent_2 = population.get(i_2);
            parents.retain(|&x| x != i_2);

            let mut individuals = [
                Individual::empty(i, individual_size),
                Individual::empty(i + 1, individual_size),
            ];

            (0..individual_size).for_each(|j| {
                if rng.gen_bool(self.rate) {
                    individuals[0].set(j, parent_1.get(j));
                    individuals[1].set(j, parent_2.get(j));
                } else {
                    individuals[0].set(j, parent_2.get(j));
                    individuals[1].set(j, parent_1.get(j));
                }
            });

            children.set(i, individuals[0].clone());
            if i + 1 < population_size {
                children.set(i + 1, individuals[1].clone());
            }
        });
        children
    }
}
