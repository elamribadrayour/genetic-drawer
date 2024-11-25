use crate::genetic_algorithm::Population;

use rand::RngCore;

pub trait Selection {
    fn select(
        &self,
        rng: &mut dyn RngCore,
        population: &Population,
        fitnesses: &[f32],
        selection_rate: f32,
    ) -> Population;
}
