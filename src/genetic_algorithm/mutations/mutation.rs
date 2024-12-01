use crate::genetic_algorithm::population::{Color, Gene, Individual, Population, Shape};

use rand::RngCore;

pub trait Mutation {
    fn mutate_shape(&self, rng: &mut dyn RngCore, shape: &mut Shape);
    fn mutate_color(&self, rng: &mut dyn RngCore, color: &mut Color);
    fn mutate_gene(&self, rng: &mut dyn RngCore, gene: &mut Gene);
    fn mutate_individual(&self, rng: &mut dyn RngCore, individual: &mut Individual);
    fn mutate(&self, rng: &mut dyn RngCore, population: &mut Population) -> usize;
}
