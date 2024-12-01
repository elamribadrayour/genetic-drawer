use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub population_size: usize,
    pub individual_size: usize,
}
