use crate::genetic_algorithm::fitnesses::{Config, Fitness, Mae, Mse, Rmse};

pub fn init(config: &Config) -> Box<dyn Fitness> {
    match config.name.as_str() {
        "mse" => Box::new(Mse) as Box<dyn Fitness>,
        "mae" => Box::new(Mae) as Box<dyn Fitness>,
        "rmse" => Box::new(Rmse) as Box<dyn Fitness>,
        _ => panic!("Invalid fitness function"),
    }
}
