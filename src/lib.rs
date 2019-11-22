#![allow(dead_code)]

extern crate rand;

mod tsp;

use tsp::TestRunner;
use tsp::solver::Grasp;
use tsp::solver::GreedySolver;
use tsp::solver::LocalSearch;
use tsp::solver::PilotSolver;
use tsp::solver::SimulatedAnnealing;
use tsp::neighborhood::DoubleEdgeExchange;
use tsp::neighborhood::DriverFlip;
use tsp::neighborhood::NeighborhoodImpl;
use tsp::neighborhood::TripleEdgeExchange;

// TODO: Kill
use tsp::Solution;
use tsp::TSPInstance;
use std::rc::Rc;

// exports
pub use tsp::neighborhood::Neighborhood;
pub use tsp::neighborhood::StepFunction;


static TIME_LIMIT: u128 = 1000 * 60 * 15;


pub fn pilot(instance_name: Option<&str>, beta: usize, runs: usize) {
    TestRunner::solve_instance(PilotSolver::new(beta), instance_name, runs);
}

pub fn greedy(instance_name: Option<&str>, candidate_size: usize, runs: usize) {
    TestRunner::solve_instance(GreedySolver::new(candidate_size), instance_name, runs);
}

pub fn local_search(instance_name: Option<&str>, neighborhood: Neighborhood, step_function: StepFunction, runs: usize) {
    match neighborhood {
        Neighborhood::DriverFlip => start_local_search(DriverFlip::new(), step_function, instance_name, runs),
        Neighborhood::DoubleEdgeExchange(x) => start_local_search(DoubleEdgeExchange::new(x), step_function, instance_name, runs),
        Neighborhood::TripleEdgeExchange(x) => start_local_search(TripleEdgeExchange::new(x), step_function, instance_name, runs),
        _ => unimplemented!()
    };
}

fn start_local_search<N> (neighborhood: N, step_function: StepFunction, instance_name: Option<&str>, runs: usize) where N: NeighborhoodImpl {
    TestRunner::solve_instance(LocalSearch::new(neighborhood, step_function), instance_name, runs);
}

pub fn grasp(instance_name: Option<&str>, candidate_size: usize, neighborhood: Neighborhood, step_function: StepFunction, runs: usize) {
    match neighborhood {
        Neighborhood::DriverFlip => start_grasp(DriverFlip::new(), step_function, candidate_size, instance_name, runs),
        Neighborhood::DoubleEdgeExchange(x) => start_grasp(DoubleEdgeExchange::new(x), step_function, candidate_size, instance_name, runs),
        Neighborhood::TripleEdgeExchange(x) => start_grasp(TripleEdgeExchange::new(x), step_function, candidate_size, instance_name, runs),
        _ => unimplemented!()
    };
}

fn start_grasp<N> (neighborhood: N, step_function: StepFunction, candidate_size: usize, instance_name: Option<&str>, runs: usize) where N: NeighborhoodImpl {
    TestRunner::solve_instance(Grasp::new(neighborhood, step_function, candidate_size), instance_name, runs);
}



pub fn vnd() {

}

pub fn simulated_annealing(instance_name: Option<&str>, neighborhoods: Vec<Neighborhood>, step_functions: Vec<StepFunction>, runs: usize) {
    TestRunner::solve_instance(SimulatedAnnealing::new(neighborhoods.iter().map(|x| get_neighborhood_impl(x)).collect(), step_functions), instance_name, runs);
}

fn get_neighborhood_impl(neighborhood: &Neighborhood) -> Box<dyn NeighborhoodImpl> {
    match neighborhood {
        Neighborhood::DoubleEdgeExchange(x) => Box::new(DoubleEdgeExchange::new(*x)),
        Neighborhood::TripleEdgeExchange(x) => Box::new(TripleEdgeExchange::new(*x)),
        Neighborhood::DriverFlip => Box::new(DriverFlip::new()),
        _ => unimplemented!()
    }
}

// TODO: Kill
pub fn test_delta() {
    let instance = Rc::new(TSPInstance::new_random(10, 4, 200, 100)); 
    let mut solution = Solution::new_random(Rc::clone(&instance));

    solution.calculate_objective_value();
    println!("Before: {}", solution.objective_value());


    DoubleEdgeExchange::apply(&mut solution, 1, 2, true);
    DriverFlip::apply(&mut solution, 2, 3, true);
    TripleEdgeExchange::apply(&mut solution, 4, 3, 3, true);
    let x = solution.objective_value();
    solution.calculate_objective_value();
    println!("With Delta: {}", x);
    println!("From distances: {}", solution.objective_value());
}
