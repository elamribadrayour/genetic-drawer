use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::Deserialize;

use crate::genetic_algorithm::crossovers::Config as CrossoverConfig;
use crate::genetic_algorithm::fitnesses::Config as FitnessConfig;
use crate::genetic_algorithm::mutations::Config as MutationConfig;
use crate::genetic_algorithm::population::Config as PopulationConfig;
use crate::genetic_algorithm::selections::Config as SelectionConfig;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub epochs: usize,
    pub log_path: String,
    pub image_path: String,
    pub fitness: FitnessConfig,
    pub mutation: MutationConfig,
    pub crossover: CrossoverConfig,
    pub selection: SelectionConfig,
    pub population: PopulationConfig,
}

impl Config {
    pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
