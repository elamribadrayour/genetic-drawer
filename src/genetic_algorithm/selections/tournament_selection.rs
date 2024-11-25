use crate::genetic_algorithm::{Population, Selection};

use rand::{Rng, RngCore};

pub struct TournamentSelection {
    pub tournament_rate: f32,
}

impl Selection for TournamentSelection {
    fn select(
        &self,
        rng: &mut dyn RngCore,
        population: &Population,
        fitnesses: &[f32],
        selection_rate: f32,
    ) -> Population {
        let nb_genes = population.individuals[0].len();
        let polygon_size = population.individuals[0].genes[0].points.len();
        let num_to_select = (population.len() as f32 * selection_rate).ceil() as usize;
        let mut selected = Population::empty(num_to_select, nb_genes, polygon_size);

        for i in 0..num_to_select {
            let tournament: Vec<usize> = (0
                ..(population.len() as f32 * self.tournament_rate).ceil() as usize)
                .map(|_| rng.gen_range(0..population.len()))
                .collect();
            let best = tournament
                .iter()
                .max_by(|a, b| fitnesses[**a].partial_cmp(&fitnesses[**b]).unwrap())
                .unwrap();
            selected.set(i, population.get(*best).clone());
        }

        selected
    }
}
