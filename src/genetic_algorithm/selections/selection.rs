use crate::genetic_algorithm::Population;

use rand::RngCore;

pub trait Selection {
    fn select(
        &self,
        rng: &mut dyn RngCore,
        population: &Population,
        fitnesses: &[f64],
        selection_rate: f64,
    ) -> Population;
}
