use crate::genetic_algorithm::crossovers::Crossover;
use crate::genetic_algorithm::population::{Individual, Population};

use rand::seq::SliceRandom;
use rand::{Rng, RngCore};

pub struct NPoint {
    pub nb_points: usize,
}

impl NPoint {
    pub fn new(nb_points: Option<usize>) -> Self {
        if nb_points.is_none() {
            panic!("nb_points is required for n-point crossover");
        }
        Self {
            nb_points: nb_points.unwrap(),
        }
    }
}

impl Crossover for NPoint {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        population: &Population,
        population_size: usize,
    ) -> Population {
        let individual_size = population.individuals[0].len();
        let mut children = Population::empty(population_size, individual_size);

        (0..population_size).step_by(2).for_each(|i| {
            let i_1 = rng.gen_range(0..population.len());
            let i_2 = rng.gen_range(0..population.len());

            let parent_1 = population.get(i_1);
            let parent_2 = population.get(i_2);

            // Generate n unique crossover points
            let mut crossover_points: Vec<usize> = (0..individual_size).collect();
            crossover_points.shuffle(rng);
            crossover_points.truncate(self.nb_points);
            crossover_points.sort_unstable();

            let mut individual_1 = Individual::empty(i, individual_size);
            let mut individual_2 = Individual::empty(i + 1, individual_size);

            let mut last_point = 0;
            for (j, &point) in crossover_points.iter().enumerate() {
                let (source_1, source_2) = if j % 2 == 0 {
                    (parent_1, parent_2)
                } else {
                    (parent_2, parent_1)
                };

                (last_point..point).for_each(|k| {
                    individual_1.set(k, source_1.get(k));
                    individual_2.set(k, source_2.get(k));
                });

                last_point = point;
            }

            // Handle the segment after the last crossover point
            let (source_1, source_2) = if crossover_points.len() % 2 == 0 {
                (parent_1, parent_2)
            } else {
                (parent_2, parent_1)
            };

            (last_point..individual_size).for_each(|k| {
                individual_1.set(k, source_1.get(k));
                individual_2.set(k, source_2.get(k));
            });

            children.set(i, individual_1);
            if i + 1 < population_size {
                children.set(i + 1, individual_2);
            }
        });
        children
    }
}
