use crate::genetic_algorithm::crossovers::{Config, Crossover, NPoint, Uniform};

pub fn init(config: &Config) -> Box<dyn Crossover> {
    match config.name.as_str() {
        "n_point" => Box::new(NPoint::new(config.nb_points)) as Box<dyn Crossover>,
        "uniform" => Box::new(Uniform::new(config.rate)) as Box<dyn Crossover>,
        _ => panic!("Invalid crossover function"),
    }
}
