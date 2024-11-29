use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub image_path: String,
    pub population_size: usize,
    pub nb_polygons: usize,
    pub fitness: String,
    pub crossover: String,
    pub mutation: String,
    pub selection: String,
    pub crossover_rate: f64,
    pub mutation_rate: f64,
    pub selection_rate: f64,
    pub polygon_size: usize,
    pub tournament_rate: Option<f64>,
    pub epochs: usize,
    pub log_path: String,
}
