use crate::genetic_algorithm::selections::{Config, Selection, TournamentSelection};

pub fn init(config: &Config) -> Box<dyn Selection> {
    match config.name.as_str() {
        "tournament" => Box::new(TournamentSelection::new(config)),
        _ => panic!("Invalid selection function"),
    }
}
