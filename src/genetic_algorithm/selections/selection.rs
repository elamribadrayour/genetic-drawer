use crate::genetic_algorithm::population::Population;

use rand::RngCore;

pub trait Selection {
    fn select(
        &self,
        rng: &mut dyn RngCore,
        population: &Population,
        fitnesses: &[f64],
    ) -> Population;
}
