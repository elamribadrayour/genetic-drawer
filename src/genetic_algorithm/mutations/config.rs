use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub rate: f64,
    pub mu: Option<f64>,
    pub sigma: Option<f64>,
}
