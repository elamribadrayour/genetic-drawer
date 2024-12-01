use rand::RngCore;

use crate::genetic_algorithm::population::Population;

pub trait Crossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        population: &Population,
        population_size: usize,
    ) -> Population;
}
