use std::{collections::HashMap, io::{self, Write}, sync::{Arc, mpsc}, thread};

pub struct Sampler;

impl Sampler {
    
    fn sample_probabilities(f: &Arc<dyn Fn() -> (usize, u16) + Sync + Send + 'static>, count: usize, thread_count: usize) -> HashMap<u16, u64> {
        let mut sampled_values: HashMap<u16, u64> = HashMap::new();

        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        let chunk_size = count / thread_count;
        for i in 0..thread_count {
            let start_chunk = i * chunk_size;
            let end_chunk = start_chunk + chunk_size;

            let tx_clone = tx.clone();
            let f = f.clone();

            let handle = thread::spawn(move || {
                for _ in start_chunk..end_chunk {
                    let sample = f();
                    
                    tx_clone.send(sample).unwrap();
                }
            });

            handles.push(handle);
        }

        drop(tx);

        let mut total_count: usize = 0;
        print!("Progress: {}%", total_count * 100 / count);
        for received in rx {
            total_count += 1;
            let _ = io::stdout().flush();
            print!("\rProgress: {}%", total_count * 100 / count);

            *sampled_values.entry(received.1).or_insert(0) += 1;
        }

        for handle in handles {
            handle.join().unwrap();
        }

        sampled_values
    }

    /// Returns the (rough) probability distribution of a quantum circuit.
    /// The quantum circuit here is represented by an ```Arc``` containing
    /// a closure that returns the result of the ```M``` method on a
    /// ```QuantumCircuit```, sampling the results of the computation
    /// ```count``` times on ```thread_count``` parallel threads.
    pub fn print_samples(f: Arc<dyn Fn() -> (usize, u16) + Sync + Send + 'static>, count: usize, thread_count: usize) {
        let probs = Self::sample_probabilities(&f, count, thread_count);
        let qubit_count = f().0;

        let mut sorted_keys: Vec<_> = probs.keys().collect();
        sorted_keys.sort();

        println!("\nSAMPLE OF QUANTUM CIRCUIT:\n");
        for i in sorted_keys {
            let value = probs.get(i).unwrap_or(&0);

            println!("\t|{:0qubit_count$b}>: {}%", i, 100.0 * (*value as f64) / (count as f64));
        }
    }

    /// Returns the (rough) probability distribution of a quantum circuit.
    /// This is the same as the ```print_samples``` method, but it prints
    /// the raw numbers instead of the qubit strings in kets.
    pub fn print_samples_raw_numbers(f: Arc<dyn Fn() -> (usize, u16) + Sync + Send + 'static>, count: usize, thread_count: usize) {
        let probs = Self::sample_probabilities(&f, count, thread_count);

        let mut sorted_keys: Vec<_> = probs.keys().collect();
        sorted_keys.sort();

        println!("\nSAMPLE OF QUANTUM CIRCUIT (raw):\n");
        for i in sorted_keys {
            let value = probs.get(i).unwrap_or(&0);

            println!("\t[{}]: {}%", i, 100.0 * (*value as f64) / (count as f64));
        }
    }
}