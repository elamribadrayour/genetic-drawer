use crate::genetic_algorithm::mutations::{Config, GaussianMutation, Mutation};

pub fn init(config: &Config) -> Box<dyn Mutation> {
    match config.name.as_str() {
        "gaussian" => Box::new(GaussianMutation::new(config)) as Box<dyn Mutation>,
        _ => panic!("Invalid mutation function"),
    }
}
