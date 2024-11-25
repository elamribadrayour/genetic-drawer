use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub image_path: String,
    pub population_size: usize,
    pub nb_genes: usize,
    pub fitness: String,
    pub crossover: String,
    pub mutation: String,
    pub selection: String,
    pub crossover_rate: f32,
    pub mutation_rate: f32,
    pub selection_rate: f32,
    pub polygon_size: usize,
    pub tournament_rate: Option<f32>,
    pub epochs: usize,
}
