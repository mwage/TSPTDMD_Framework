#![allow(dead_code)]

extern crate rand;

mod tsp;

use tsp::TestRunner;
use tsp::solver::Grasp;
use tsp::solver::GreedySolver;
use tsp::solver::LocalSearch;
use tsp::solver::PilotSolver;
use tsp::solver::SimulatedAnnealing;
use tsp::solver::VariableNeighborhood;
use tsp::neighborhood::DoubleEdgeExchange;
use tsp::neighborhood::DriverFlip;
use tsp::neighborhood::NeighborhoodImpl;
use tsp::neighborhood::TripleEdgeExchange;

// exports
pub use tsp::neighborhood::Neighborhood;
pub use tsp::neighborhood::StepFunction;


static TIME_LIMIT: u128 = 1000 * 60;


pub fn pilot(instance_name: Option<&str>, beta: usize, runs: usize) {
    TestRunner::solve_instance(PilotSolver::new(beta), instance_name, runs);
}

pub fn greedy(instance_name: Option<&str>, candidate_size: usize, runs: usize) {
    TestRunner::solve_instance(GreedySolver::new(candidate_size), instance_name, runs);
}

pub fn local_search(instance_name: Option<&str>, neighborhood: Neighborhood, step_function: StepFunction, iteration_limit: usize, runs: usize) {
    match neighborhood {
        Neighborhood::DriverFlip => start_local_search(DriverFlip::new(), step_function, iteration_limit, instance_name, runs),
        Neighborhood::DoubleEdgeExchange(x) => start_local_search(DoubleEdgeExchange::new(x), step_function, iteration_limit, instance_name, runs),
        Neighborhood::TripleEdgeExchange(x) => start_local_search(TripleEdgeExchange::new(x), step_function, iteration_limit, instance_name, runs),
        _ => unimplemented!()
    };
}

fn start_local_search<N> (neighborhood: N, step_function: StepFunction, iteration_limit: usize, instance_name: Option<&str>, runs: usize) where N: NeighborhoodImpl {
    TestRunner::solve_instance(LocalSearch::new(neighborhood, step_function, iteration_limit), instance_name, runs);
}

pub fn grasp(instance_name: Option<&str>, candidate_size: usize, neighborhood: Neighborhood, step_function: StepFunction, iteration_limit: usize, ls_iteration_limit: usize, runs: usize) {
    match neighborhood {
        Neighborhood::DriverFlip => start_grasp(DriverFlip::new(), step_function, candidate_size, iteration_limit, ls_iteration_limit, instance_name, runs),
        Neighborhood::DoubleEdgeExchange(x) => start_grasp(DoubleEdgeExchange::new(x), step_function, candidate_size, iteration_limit, ls_iteration_limit, instance_name, runs),
        Neighborhood::TripleEdgeExchange(x) => start_grasp(TripleEdgeExchange::new(x), step_function, candidate_size, iteration_limit, ls_iteration_limit, instance_name, runs),
        _ => unimplemented!()
    };
}

fn start_grasp<N> (neighborhood: N, step_function: StepFunction, candidate_size: usize, iteration_limit: usize, ls_iteration_limit: usize, instance_name: Option<&str>, runs: usize) where N: NeighborhoodImpl {
    TestRunner::solve_instance(Grasp::new(neighborhood, step_function, candidate_size, iteration_limit, ls_iteration_limit), instance_name, runs);
}

pub fn variable_neighborhood(instance_name: Option<&str>, neighborhoods: Vec<Neighborhood>, runs: usize) {
    TestRunner::solve_instance(VariableNeighborhood::new(neighborhoods.iter().map(|x| get_neighborhood_impl(x)).collect()), instance_name, runs);
}

pub fn simulated_annealing(instance_name: Option<&str>, neighborhoods: Vec<Neighborhood>, step_functions: Vec<StepFunction>, runs: usize) {
    // TestRunner::solve_instance(SimulatedAnnealing::new(neighborhoods.iter().map(|x| get_neighborhood_impl(x)).collect(), step_functions), instance_name, runs);
}

fn get_neighborhood_impl(neighborhood: &Neighborhood) -> Box<dyn NeighborhoodImpl> {
    match neighborhood {
        Neighborhood::DoubleEdgeExchange(x) => Box::new(DoubleEdgeExchange::new(*x)),
        Neighborhood::TripleEdgeExchange(x) => Box::new(TripleEdgeExchange::new(*x)),
        Neighborhood::DriverFlip => Box::new(DriverFlip::new()),
        _ => unimplemented!()
    }
}

// Returns positive modulo
pub fn modulo_pos(number: isize, module: usize) -> usize {
    let module = module as isize;
    let x = number % module;
    if x < 0 {
        (x + module) as usize
    } else {
        x as usize
    }
}
