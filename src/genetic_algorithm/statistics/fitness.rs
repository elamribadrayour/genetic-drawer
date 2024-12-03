use serde_json::{json, Value};

pub struct Fitness;

impl Fitness {
    pub fn get_mean(fitnesses: &[f64]) -> f64 {
        let mean = fitnesses.iter().sum::<f64>() / fitnesses.len() as f64;
        (mean * 100.0).round() / 100.0
    }

    pub fn get_std_dev(fitnesses: &[f64]) -> f64 {
        let mean = Self::get_mean(fitnesses);
        let variance =
            fitnesses.iter().map(|f| (f - mean).powi(2)).sum::<f64>() / fitnesses.len() as f64;
        (variance * 100.0).round() / 100.0
    }

    pub fn get_min(fitnesses: &[f64]) -> Value {
        let (index, fitness) = fitnesses
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        let fitness = (fitness * 100.0).round() / 100.0;
        json!({
            "index": index,
            "value": fitness,
        })
    }

    pub fn get_max(fitnesses: &[f64]) -> Value {
        let (index, fitness) = fitnesses
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        json!({
            "index": index,
            "value": fitness,
        })
    }

    pub fn get_median(fitnesses: &[f64]) -> f64 {
        let mut sorted = fitnesses.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = sorted[sorted.len() / 2];
        (median * 100.0).round() / 100.0
    }

    pub fn get(fitnesses: &[f64]) -> Value {
        json!({
            "min": Self::get_min(fitnesses),
            "max": Self::get_max(fitnesses),
            "mean": Self::get_mean(fitnesses),
            "median": Self::get_median(fitnesses),
            "std-dev": Self::get_std_dev(fitnesses),
        })
    }
}
