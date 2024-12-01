use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Duration;

use plotly::{Plot, Scatter};
use serde_json::json;
use serde_json::Value;

use crate::genetic_algorithm::config::Config;
use crate::genetic_algorithm::statistics;

pub struct Logger {
    pub logger: BufWriter<File>,
    pub plots: HashMap<String, Plot>,
}

impl Logger {
    pub fn new(config: &Config) -> Self {
        let file = File::create(&config.log_path).unwrap();
        let logger = BufWriter::new(file);
        let plots = ["mutated", "fitness"]
            .iter()
            .map(|name| (name.to_string(), Plot::new()))
            .collect();

        if std::fs::metadata(".cache").is_ok() {
            std::fs::remove_dir_all(".cache").unwrap();
        }

        std::fs::create_dir_all(".cache/best").unwrap();
        std::fs::create_dir_all(".cache/plots").unwrap();
        std::fs::create_dir_all(".cache/frames").unwrap();

        Self { logger, plots }
    }

    pub fn log_file(&mut self, log: &Value) {
        self.logger
            .write_all((log.to_string() + "\n").as_bytes())
            .unwrap();
    }

    pub fn log_stdout(&mut self, log: &Value, epoch: &usize, epochs: &usize) {
        if *epochs > 100 && epoch % (epochs / 100) != 0 {
            return;
        }

        println!(
            "epochs: {} - epoch: {} - fitness: {}",
            epochs,
            epoch,
            log["fitness"]["max"]["value"].as_f64().unwrap()
        );
    }

    pub fn log_best(&mut self, log: &Value, epoch: &usize, epochs: &usize) {
        if *epochs > 100 && epoch % (epochs / 100) != 0 {
            return;
        }

        let index = log["fitness"]["max"]["index"].as_u64().unwrap() as usize;
        let target_path = format!(".cache/best/{}.png", epoch);
        let source_path = format!(".cache/frames/{}.png", index);
        std::fs::copy(source_path, target_path).unwrap();
    }

    pub fn log_plot(&mut self, log: &Value, epoch: &usize, epochs: &usize) {
        if *epochs > 100 && epoch % (epochs / 100) != 0 {
            return;
        }

        let x = vec![*epoch as f64];

        let y_fitness = vec![log["fitness"]["max"]["value"].as_f64().unwrap()];
        let y_mutated = vec![log["mutated"].as_u64().unwrap() as f64];

        self.plots
            .get_mut("fitness")
            .unwrap()
            .add_trace(Scatter::new(x.clone(), y_fitness));
        self.plots
            .get_mut("mutated")
            .unwrap()
            .add_trace(Scatter::new(x, y_mutated));
    }

    pub fn log(
        &mut self,
        epochs: &usize,
        epoch: &usize,
        mutated: &usize,
        fitnesses: &[f64],
        durations: &HashMap<&str, Duration>,
    ) {
        let durations: serde_json::Value = durations
            .iter()
            .map(|(name, duration)| {
                json!({ name.to_string(): (1000.0 * duration.as_secs_f64()) as u64 })
            })
            .collect();

        let log = json!({
            "epoch": epoch,
            "mutated": mutated,
            "duration": durations,
            "fitness": statistics::Fitness::get(fitnesses),
        });

        self.log_file(&log);
        self.log_stdout(&log, epoch, epochs);
        self.log_best(&log, epoch, epochs);
        self.log_plot(&log, epoch, epochs);
    }

    pub fn flush(&mut self) {
        self.logger.flush().unwrap();
        for (name, plot) in self.plots.iter_mut() {
            plot.write_html(format!(".cache/plots/{}.html", name));
        }
    }
}
