use crate::genetic_algorithm::Population;

use rand::RngCore;

pub trait Crossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        population: &Population,
        crossover_rate: f32,
        population_size: usize,
    ) -> Population;
}
