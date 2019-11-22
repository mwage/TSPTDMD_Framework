use std::fs;
use std::rc::Rc;

use crate::tsp::io::InstanceParser;
use crate::tsp::io::Logger;
use crate::tsp::solver::Solver;

pub struct TestRunner {
}

impl TestRunner {
    pub fn solve_instance<T: Solver>(mut solver: T, instance_name: Option<&str>) {
        if let Some(instance) = instance_name {
            TestRunner::run_instance(&mut solver, instance)  // Solve a given instance
        } else {
            TestRunner::run_all_instances(solver); // Solve all instances
        }
    }

    fn run_all_instances<T: Solver>(mut solver: T) {
        let paths = fs::read_dir("instances").unwrap(); // Get all file paths in the instances folder
        for path in paths {
            let path_buff = path.unwrap().path();   // Get path
            let instance_name = path_buff.to_str().unwrap(); // Get instance name from path            
            let separator = if instance_name.contains("\\") {
                '\\'
            } else {
                '/'
            };
            let instance_name = instance_name.split(separator).last().unwrap().split('.').next().unwrap();
            TestRunner::run_instance(&mut solver, instance_name);   // Solve the given instance
        }
    }

    fn run_instance<T: Solver>(solver: &mut T, instance_name: &str) {
        let logger = Logger::new(solver, instance_name);    // Initialize logger, starts the timer
        let instance = match InstanceParser::get_instance(instance_name) // Parse the instance from file
        {
            Ok(x) => x,
            Err(_) => {
                println!("Skipping {}.txt: Failed to read instance.", instance_name);
                return;
            },
        };
        println!("Solve instance: {}", instance_name);
        solver.solve(Rc::new(instance), logger);   // Solve TSP instance with selected solver    
    }
}