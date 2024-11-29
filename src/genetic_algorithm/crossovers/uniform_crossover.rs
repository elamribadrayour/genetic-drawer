use crate::genetic_algorithm::{Crossover, Individual, Population};

use rand::{Rng, RngCore};

pub struct UniformCrossover;

impl Crossover for UniformCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        population: &Population,
        crossover_rate: f64,
        population_size: usize,
    ) -> Population {
        let nb_polygons = population.individuals[0].len();
        let polygon_size = population.individuals[0].genes[0].values.len();
        let mut children = Population::empty(population_size, nb_polygons, polygon_size);

        (0..population_size).for_each(|i| {
            let i_1 = rng.gen_range(0..population.len());
            let i_2 = rng.gen_range(0..population.len());

            let parent_1 = population.get(i_1);
            let parent_2 = population.get(i_2);

            let mut individual_1 = Individual::empty(i, nb_polygons, polygon_size);
            let mut individual_2 = Individual::empty(i + 1, nb_polygons, polygon_size);

            (0..nb_polygons).for_each(|j| {
                if rng.gen_bool(crossover_rate) {
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
