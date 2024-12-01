use crate::genetic_algorithm::crossovers::{Config, Crossover, UniformCrossover};

pub fn init(config: &Config) -> Box<dyn Crossover> {
    match config.name.as_str() {
        "uniform" => Box::new(UniformCrossover::new(config.rate)) as Box<dyn Crossover>,
        _ => panic!("Invalid crossover function"),
    }
}
