use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub shapes: Vec<String>,
    pub population_size: usize,
    pub individual_size: usize,
}
