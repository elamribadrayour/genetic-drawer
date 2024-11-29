use crate::genetic_algorithm::{Gene, Individual, Population};

use rand::RngCore;

pub trait Mutation {
    fn mutate_gene(&self, rng: &mut dyn RngCore, gene: &mut Gene);
    fn mutate_individual(&self, rng: &mut dyn RngCore, individual: &mut Individual);
    fn mutate(
        &self,
        rng: &mut dyn RngCore,
        population: &mut Population,
        mutation_rate: f64,
    ) -> usize;
}
