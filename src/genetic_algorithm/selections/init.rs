use crate::genetic_algorithm::selections::{Config, Elite, Selection, Tournament};

pub fn init(config: &Config) -> Box<dyn Selection> {
    match config.name.as_str() {
        "tournament" => Box::new(Tournament::new(config)),
        "elite" => Box::new(Elite::new(config)),
        _ => panic!("Invalid selection function"),
    }
}
