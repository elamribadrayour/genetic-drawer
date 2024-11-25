use crate::genetic_algorithm::Population;

use rand::RngCore;

pub trait Mutation {
    fn mutate(&self, rng: &mut dyn RngCore, population: &mut Population, mutation_rate: f32);
}
