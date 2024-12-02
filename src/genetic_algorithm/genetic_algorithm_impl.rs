use crate::genetic_algorithm::*;

use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

use image::RgbImage;
use rand::{thread_rng, RngCore};
use rayon::prelude::*;

pub struct GeneticAlgorithm {
    pub epoch: usize,
    pub epochs: usize,
    pub source: RgbImage,
    pub rng: Box<dyn RngCore>,
    pub population: population::Population,
    pub fitness: Box<dyn fitnesses::Fitness>,
    pub mutation: Box<dyn mutations::Mutation>,
    pub crossover: Box<dyn crossovers::Crossover>,
    pub selection: Box<dyn selections::Selection>,
    pub population_size: usize,
    pub logger: statistics::Logger,
}

impl GeneticAlgorithm {
    pub fn new(config: config::Config) -> Result<Self, Box<dyn Error>> {
        std::fs::create_dir_all(".cache/frames").unwrap();

        let epoch = 0;
        let mut rng = thread_rng();
        let logger = statistics::Logger::new(&config);
        let fitness = fitnesses::init(&config.fitness);
        let mutation = mutations::init(&config.mutation);
        let crossover = crossovers::init(&config.crossover);
        let selection = selections::init(&config.selection);
        let population = population::Population::new(&config.population, &mut rng);
        let source = image::open(&config.image_path).unwrap().to_rgb8();

        Ok(GeneticAlgorithm {
            epoch,
            epochs: config.epochs,
            source,
            rng: Box::new(rng),
            population,
            fitness,
            crossover,
            mutation,
            selection,
            population_size: config.population.population_size,
            logger,
        })
    }

    pub fn draw(&mut self) {
        self.population.par_iter().for_each(|individual| {
            individual.draw(&self.source.dimensions());
        });
    }

    pub fn read(&self) -> Vec<RgbImage> {
        self.population
            .par_iter()
            .map(|individual| individual.read())
            .collect()
    }

    pub fn evaluate(&self, images: &[RgbImage]) -> Vec<f64> {
        images
            .iter()
            .map(|image| self.fitness.calculate(&self.source, image))
            .collect()
    }

    pub fn select(&mut self, fitnesses: &[f64]) {
        self.selection
            .select(&mut *self.rng, &self.population, fitnesses);
    }

    pub fn crossover(&mut self) {
        self.population =
            self.crossover
                .crossover(&mut *self.rng, &self.population, self.population_size);
    }

    pub fn mutation(&mut self) -> usize {
        self.mutation.mutate(&mut *self.rng, &mut self.population)
    }

    pub fn update(&mut self) {
        let mut durations = HashMap::new();

        let time = Instant::now();
        self.draw();
        durations.insert("draw", time.elapsed());

        let time = Instant::now();
        let images = self.read();
        durations.insert("read", time.elapsed());

        let time = Instant::now();
        let fitnesses = self.evaluate(&images);
        durations.insert("evaluate", time.elapsed());

        let time = Instant::now();
        self.select(&fitnesses);
        durations.insert("select", time.elapsed());

        let time = Instant::now();
        self.crossover();
        durations.insert("crossover", time.elapsed());

        let time = Instant::now();
        let mutated = self.mutation();
        durations.insert("mutation", time.elapsed());

        self.logger
            .log(&self.epochs, &self.epoch, &mutated, &fitnesses, &durations);

        let fitness = statistics::Fitness::get_max(&fitnesses);
        let value = fitness["value"].as_f64().unwrap();
        let index = fitness["index"].as_u64().unwrap() as usize;

        if (value - 1.0).abs() < f64::EPSILON || self.epochs == self.epoch + 1 {
            let target_path = ".cache/result.png".to_string();
            let source_path = format!(".cache/frames/{}.png", index);
            std::fs::copy(source_path, target_path).unwrap();
            self.logger.flush();
            return;
        }

        self.epoch += 1;
    }

    pub fn run(&mut self) {
        (0..self.epochs).for_each(|_| self.update());
    }
}
