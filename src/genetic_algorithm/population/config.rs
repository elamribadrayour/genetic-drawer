use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub shapes: Vec<String>,
    pub max_area: Option<f64>,
    pub population_size: usize,
    pub individual_size: usize,
    pub color_type: Option<String>,
}
