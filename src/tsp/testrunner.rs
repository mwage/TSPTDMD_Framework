use std::fs;

use crate::tsp::io::InstanceParser;
use crate::tsp::io::Logger;
use crate::tsp::solver::Solver;

pub struct TestRunner {
}

impl TestRunner {
    pub fn solve_instance<T: Solver>(mut solver: T, instance_name: Option<&str>) {
        if let Some(instance) = instance_name {
            println!("{}", instance);
            TestRunner::run_instance(&mut solver, instance)  // Solve a given instance
        } else {
            println!("All");
            TestRunner::run_all_instances(solver); // Solve all instances
        }
    }

    fn run_all_instances<T: Solver>(mut solver: T) {
        let paths = fs::read_dir("instances").unwrap(); // Get all file paths in the instances folder
        for path in paths {
            TestRunner::run_instance(&mut solver, path.unwrap().path().to_str().unwrap().split('\\').last().unwrap());   // Get instance name from path
        }
    }

    fn run_instance<T: Solver>(solver: &mut T, instance_name: &str) {
        let logger = Logger::new(solver, instance_name);    // Initialize logger, starts the timer
        let mut instance = InstanceParser::get_instance(instance_name); // Parse the instance from file
        solver.solve(&mut instance, &logger);   // Solve TSP instance with selected solver
        logger.log_result(instance);    // Log results to file
    }
}