use std::fs;

use crate::tsp::TSPInstance;
use crate::Algorithm;
use crate::tsp::Logger;
use crate::tsp::algorithms::GreedySolver;
use crate::tsp::algorithms::RandomGreedySolver;
use crate::tsp::algorithms::Solver;
use crate::tsp::InstanceParser;
use crate::tsp::algorithms::NeighborhoodFunction;

pub struct TestRunner {
}

impl TestRunner {    
    // Select the solver and use it to solve the given instance
    fn select_solver(algorithm: &Algorithm, neighborhoods: Option<Vec<NeighborhoodFunction>>) -> Vec<impl Solver> {
        let mut solver = Vec::new();
        match algorithm {
            Algorithm::Greedy => solver.push(GreedySolver::new()),
            Algorithm::RandomGreedy => solver.push(RandomGreedySolver::new())
        };
        solver
    }

    pub fn setup(algorithm: &Algorithm, neighborhoods: Option<Vec<NeighborhoodFunction>>) {
        let x = TestRunner::select_solver(algorithm, neighborhoods);
        for solver in x.iter() {
            TestRunner::test(solver);
        }
    }

    pub fn test<T: Solver>(solver: &T) {

    }

    pub fn run_all_instances(algorithm: Algorithm) {
        let paths = fs::read_dir("instances").unwrap(); // Get all file paths in the instances folder
        for path in paths {
            TestRunner::run_instance(&algorithm, path.unwrap().path().to_str().unwrap().split('\\').last().unwrap());   // Get instance name from path
        }
    }

    pub fn run_instance(algorithm: &Algorithm, instance_name: &str) {
        let logger = Logger::new(&algorithm, instance_name);    // Initialize logger, starts the timer
        let mut instance = InstanceParser::get_instance(instance_name); // Parse the instance from file
        TestRunner::select_solver(algorithm, &mut instance, &logger);   // Solve TSP instance with selected solver
        logger.log_result(instance);    // Log results to file
    }

    
    // Solve the instance
    fn run<T>(instance: &mut TSPInstance, mut solver: T, logger: &Logger) where T: Solver {
        solver.solve(instance, logger);
    }
}