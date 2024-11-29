use crate::genetic_algorithm::{
    statistics, Config, Crossover, Fitness, GaussianMutation, Mae, Mse, Mutation, Population, Rmse,
    Selection, TournamentSelection, UniformCrossover,
};

use std::fs::File;
use std::io::{BufWriter, Write};

use image::RgbaImage;
use rand::{thread_rng, RngCore};
use rayon::prelude::*;
use serde_json::json;

pub struct GeneticAlgorithm {
    pub epoch: usize,
    pub epochs: usize,
    pub source: RgbaImage,
    pub rng: Box<dyn RngCore>,
    pub population: Population,
    pub fitness: Box<dyn Fitness>,
    pub mutation: Box<dyn Mutation>,
    pub crossover: Box<dyn Crossover>,
    pub selection: Box<dyn Selection>,
    pub mutation_rate: f64,
    pub crossover_rate: f64,
    pub selection_rate: f64,
    pub population_size: usize,
    pub logger: BufWriter<File>,
}

impl GeneticAlgorithm {
    pub fn new(config: Config) -> Self {
        let iter = 0;
        let mut rng = thread_rng();
        let population = Population::new(
            &mut rng,
            config.population_size,
            config.nb_polygons,
            config.polygon_size,
        );
        let source = image::open(config.image_path).unwrap().to_rgba8();

        let fitness = match config.fitness.as_str() {
            "mse" => Box::new(Mse) as Box<dyn Fitness>,
            "mae" => Box::new(Mae) as Box<dyn Fitness>,
            "rmse" => Box::new(Rmse) as Box<dyn Fitness>,
            _ => panic!("Invalid fitness function"),
        };

        let mutation = match config.mutation.as_str() {
            "gaussian" => Box::new(GaussianMutation {
                mu: 0.0,
                sigma: 0.05,
            }) as Box<dyn Mutation>,
            _ => panic!("Invalid mutation function"),
        };

        let crossover = match config.crossover.as_str() {
            "uniform" => Box::new(UniformCrossover) as Box<dyn Crossover>,
            _ => panic!("Invalid crossover function"),
        };

        let selection = match config.selection.as_str() {
            "tournament" => Box::new(TournamentSelection {
                tournament_rate: config.tournament_rate.unwrap_or(0.5),
            }),
            _ => panic!("Invalid selection function"),
        };

        std::fs::create_dir_all(".cache/frames").unwrap();
        std::fs::create_dir_all(".cache/best").unwrap();

        let log_file = File::create(config.log_path).unwrap();
        let logger = BufWriter::new(log_file);

        GeneticAlgorithm {
            epoch: iter,
            epochs: config.epochs,
            source,
            rng: Box::new(rng),
            population,
            fitness,
            crossover,
            mutation,
            selection,
            crossover_rate: config.crossover_rate,
            mutation_rate: config.mutation_rate,
            selection_rate: config.selection_rate,
            population_size: config.population_size,
            logger,
        }
    }

    pub fn draw(&mut self) {
        self.population.par_iter().for_each(|individual| {
            individual.draw(&self.source.dimensions());
        });
    }

    pub fn read(&self) -> Vec<RgbaImage> {
        self.population
            .par_iter()
            .map(|individual| individual.read())
            .collect()
    }

    pub fn evaluate(&self, images: &[RgbaImage]) -> Vec<f64> {
        images
            .iter()
            .map(|image| self.fitness.calculate(&self.source, image))
            .collect()
    }

    pub fn select(&mut self, fitnesses: &[f64]) {
        self.selection.select(
            &mut *self.rng,
            &self.population,
            fitnesses,
            self.selection_rate,
        );
    }

    pub fn crossover(&mut self) {
        self.population = self.crossover.crossover(
            &mut *self.rng,
            &self.population,
            self.crossover_rate,
            self.population_size,
        );
    }

    pub fn mutation(&mut self) -> usize {
        self.mutation
            .mutate(&mut *self.rng, &mut self.population, self.mutation_rate)
    }

    pub fn update(&mut self) {
        self.draw();
        let images = self.read();
        let fitnesses = self.evaluate(&images);
        self.select(&fitnesses);
        self.crossover();
        let mutated = self.mutation();

        let log = json!({
            "epoch": self.epoch,
            "mutated": mutated,
            "fitness": statistics::Fitness::get(&fitnesses),
            "image": statistics::Image::get(self.population_size),
        });

        self.logger
            .write_all((log.to_string() + "\n").as_bytes())
            .unwrap();

        if self.epoch % (self.epochs / 10) == 0 {
            println!(
                "epoch: {} - fitness: {}",
                log["epoch"].as_u64().unwrap(),
                log["fitness"]["max"]["value"].as_f64().unwrap()
            );
        }

        let max_fitness = log["fitness"]["max"].as_object().unwrap();
        let index = max_fitness["index"].as_u64().unwrap() as usize;
        let target_path = format!(".cache/best/{}.png", self.epoch);
        let source_path = format!(".cache/frames/{}.png", index);
        std::fs::copy(source_path, target_path).unwrap();

        if (max_fitness["value"].as_f64().unwrap() - 1.0).abs() < f64::EPSILON
            || self.epochs == self.epoch + 1
        {
            let target_path = ".cache/result.png".to_string();
            let source_path = format!(".cache/frames/{}.png", index);
            std::fs::copy(source_path, target_path).unwrap();
            self.logger.flush().unwrap();
            return;
        }

        self.epoch += 1;
    }

    pub fn run(&mut self) {
        (0..self.epochs).for_each(|_| self.update());
    }
}
