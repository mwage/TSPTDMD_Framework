use crate::tsp::TSPInstance;
use crate::tsp::solver::Solver;
use std::time::Instant;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Write, BufWriter, Error};

pub struct Logger {
    timer: Instant,
    path: String
}

impl Logger {
    pub fn new<T: Solver >(solver: &T, instance_name: &str) -> Logger {
        fs::create_dir_all(format!("results/{}", solver.to_string())).expect("Failed to create directories.");
        Logger {
            timer: Instant::now(),
            path: String::from(format!("results/{}/{}", solver.to_string(), instance_name)) // TODO: Properly set log path
        }
    }

    pub fn get_elapsed(&self) -> u128 {
        self.timer.elapsed().as_millis()
    }

    pub fn log_result(&self, instance: TSPInstance) {
        let mut result_strings = Vec::new();

        // TODO: Get results from instance and format according to guidelines
        self.to_file(result_strings).expect("Failed to log to file.");
    }

    // Writes the given result strings to file
    fn to_file(&self, result_strings: Vec<String>) -> Result<(), Error> {
        let f = OpenOptions::new().write(true).create(true).append(false).open(&self.path)?;
        let mut f = BufWriter::new(f);
        for result in result_strings.iter() {
            f.write_all(result.as_bytes())?;
        }

        Ok(())
    }
}