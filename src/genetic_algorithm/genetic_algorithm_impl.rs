use crate::genetic_algorithm::{
    Config, Crossover, Fitness, GaussianMutation, Mae, Mse, Mutation, Population, Rmse, Selection,
    TournamentSelection, UniformCrossover,
};

use std::time::Instant;

use image::RgbaImage;
use rand::{thread_rng, RngCore};
use rayon::prelude::*;

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
    pub mutation_rate: f32,
    pub crossover_rate: f32,
    pub selection_rate: f32,
    pub population_size: usize,
}

impl GeneticAlgorithm {
    pub fn new(config: Config) -> Self {
        let iter = 0;
        let mut rng = thread_rng();
        let population = Population::new(
            &mut rng,
            config.population_size,
            config.nb_genes,
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
                sigma: 1.0,
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

        std::fs::create_dir_all(".cache").unwrap();

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

    pub fn evaluate(&self, images: &[RgbaImage]) -> Vec<f32> {
        images
            .iter()
            .map(|image| self.fitness.calculate(&self.source, image))
            .collect()
    }

    pub fn select(&mut self, fitnesses: &[f32]) {
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

    pub fn mutation(&mut self) {
        self.mutation
            .mutate(&mut *self.rng, &mut self.population, self.mutation_rate);
    }

    pub fn update(&mut self) {
        let start = Instant::now();
        self.draw();
        let draw_time = start.elapsed();

        // Get generate image
        let start = Instant::now();
        let images = self.read();
        let get_images_time = start.elapsed();

        // Calculate fitness of the generated images using the fitness function
        let start = Instant::now();
        let fitnesses = self.evaluate(&images);
        let fitness_time = start.elapsed();

        // Time each step and log at the end
        self.select(&fitnesses);
        let selection_time = start.elapsed();

        let start = Instant::now();
        self.crossover();
        let crossover_time = start.elapsed();

        let start = Instant::now();
        self.mutation();
        let mutation_time = start.elapsed();

        // Update iteration
        self.epoch += 1;

        let fitness = fitnesses
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        if self.epoch % 1000 == 0 {
            println!(
                "iteration: {}\tsize: {}\tfitness: {}\tdraw: {:?}s\tget_images: {:?}s\tfitness: {:?}s\tselection: {:?}s\tcrossover: {:?}s\tmutation: {:?}s",
            self.epoch,
            self.population.len(),
            fitness,
            draw_time.as_secs_f32(),
            get_images_time.as_secs_f32(),
            fitness_time.as_secs_f32(),
            selection_time.as_secs_f32(),
                crossover_time.as_secs_f32(),
                mutation_time.as_secs_f32()
            );
        }
    }

    pub fn run(&mut self) {
        (0..self.epochs).for_each(|_| self.update());
    }
}
