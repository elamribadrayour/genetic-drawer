use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub name: String,
    pub rate: f64,
}
