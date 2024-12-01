use std::error::Error;

mod genetic_algorithm;
use genetic_algorithm::Config;
use genetic_algorithm::GeneticAlgorithm;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new("Config.json")?;
    let mut ga = GeneticAlgorithm::new(config)?;
    ga.run();
    Ok(())
}
