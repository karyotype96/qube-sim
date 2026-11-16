use std::collections::HashMap;

pub struct Sampler;

impl Sampler {
    pub fn sample_probabilities(f: &impl Fn() -> Vec<u8>, count: usize) -> HashMap<u8, u64> {
        let mut sampled_values: HashMap<u8, u64> = HashMap::new();

        for _ in 0..count {
            let sample = f();
            let index = sample.iter()
                .enumerate()
                .fold(0, |acc, (index, &value)| acc + (1 << index) * value);
            
            *sampled_values.entry(index).or_insert(0) += 1;
        }

        sampled_values
    }

    pub fn print_samples(f: impl Fn() -> Vec<u8>, count: usize) {
        let probs = Self::sample_probabilities(&f, count);
        let qubit_count = f().len();

        let mut sorted_keys: Vec<_> = probs.keys().collect();
        sorted_keys.sort();

        println!("SAMPLE OF QUANTUM CIRCUIT:\n");
        for i in sorted_keys {
            let value = probs.get(i).unwrap_or(&0);

            println!("\t|{:0qubit_count$b}>: {}%", i, 100.0 * (*value as f64) / (count as f64));
        }
    }
}