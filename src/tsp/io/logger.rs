use std::time::Instant;
use std::fs;
use std::fs::OpenOptions;
use std::io::{Write, BufWriter, Error};
use std::path::Path;

use crate::tsp::solver::Solver;
use crate::tsp::Solution;



pub struct Logger {
    timer: Instant,
    instance_name: String,
    path: String
}

impl Logger {
    pub fn new<T: Solver >(solver: &T, instance_name: &str) -> Logger {
        fs::create_dir_all(format!("results/{}", solver.to_string())).expect("Failed to create directories.");  // Create log directory if non existant
        let mut counter = 0;
        let mut path = String::from(format!("results/{}/{}", solver.to_string(), instance_name));
        loop {
            let log_path = format!("{}.{}.txt", path, counter);
            if Path::new(&log_path).exists() {
                counter += 1;
                continue;
            }

            path = log_path;
            break;
        }

        Logger {
            timer: Instant::now(),  // Start timer
            instance_name: String::from(instance_name),
            path
        }
    }

    pub fn get_elapsed(&self) -> u128 { // elapsed time since start in ms
        self.timer.elapsed().as_millis()
    }

    pub fn log_result(&self, solution: &Solution) {
        let mut result_strings = Vec::new();
        result_strings.push(self.instance_name.to_owned());
        result_strings.push(solution.vertices_to_str());
        result_strings.push(solution.drivers_to_str());
        result_strings.push(format!("val: {}", solution.objective_value()));
        result_strings.push(format!("{}ms", self.get_elapsed()));
        result_strings.push(format!("TODO: VALID"));
        self.to_file(result_strings).expect("Failed to log to file.");
    }

    // Writes the given result strings to file
    fn to_file(&self, result_strings: Vec<String>) -> Result<(), Error> {
        let f = OpenOptions::new().write(true).create(true).append(false).open(&self.path)?;
        let mut f = BufWriter::new(f);
        for result in result_strings.iter() {
            f.write_all(format!("{}\r\n", result).as_bytes())?;
        }

        Ok(())
    }
}