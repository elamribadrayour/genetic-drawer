mod genetic_algorithm;
use genetic_algorithm::{Config, GeneticAlgorithm};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let file = File::open("Config.json").unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).unwrap();
    let mut ga = GeneticAlgorithm::new(config);
    ga.run();
}
