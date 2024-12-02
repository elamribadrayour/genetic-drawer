use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub rate: Option<f64>,
    pub nb_points: Option<usize>,
}
