# Genetic Drawer

Genetic Drawer is a Rust-based project inspired by EvoLISA, designed to recreate images using a genetic algorithm. The project evolves a population of polygons to approximate a target image, such as the Mona Lisa, through iterative processes of selection, crossover, and mutation.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [License](#license)
- [Contact](#contact)

## Introduction

This project is a genetic algorithm implementation that attempts to recreate a given image using a set of polygons. The algorithm iteratively improves the population of images by applying genetic operations such as selection, crossover, and mutation.

## Features

- **Configurable Genetic Algorithm**: Customize the algorithm's parameters through a JSON configuration file.
- **Multiple Fitness Functions**: Choose from Mean Absolute Error (MAE), Mean Squared Error (MSE), and Root Mean Squared Error (RMSE) to evaluate image similarity.
- **Crossover and Mutation**: Implementations of uniform crossover and Gaussian mutation to evolve the population.
- **Parallel Processing**: Utilizes Rayon for parallel processing to speed up computations.

## Installation

To build and run the project, ensure you have Rust installed. Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/elamribadrayour/genetic-drawer
cd genetic-drawer
cargo build --release
````

## Usage

To run the genetic drawer, execute the following command:

```bash
cargo run --release
```


This will start the genetic algorithm using the configuration specified in `Config.json`.

## Configuration

The behavior of the genetic algorithm can be customized using the `Config.json` file. Below is an example configuration:

```json
{
    "image_path": "assets/images/mona-lisa.jpg",
    "population_size": 100,
    "nb_polygons": 50,
    "fitness": "mse",
    "crossover": "uniform",
    "mutation": "gaussian",
    "selection": "tournament",
    "crossover_rate": 0.7,
    "mutation_rate": 0.01,
    "selection_rate": 0.5,
    "polygon_size": 6,
    "tournament_rate": 0.5,
    "epochs": 10000
}
```


## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any inquiries or feedback, please contact me at [badrayour.elamri@protonmail.com](mailto:badrayour.elamri@protonmail.com).
Feel free to adjust the content to better fit your project's specifics or to add any additional sections you find necessary.
