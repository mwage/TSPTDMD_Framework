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
use rand::Rng;
use tsp::io::Logger;
use tsp::io::InstanceParser;
use tsp::solver::Solver;

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

    let mut instance =  TSPInstance::new(6, 2, 0);
    instance.add_edge(0, 1, 1);
    instance.add_edge(0, 2, 20);
    instance.add_edge(0, 3, 4);
    instance.add_edge(0, 4, 5);
    instance.add_edge(0, 5, 6);
    instance.add_edge(1, 2, 2);
    instance.add_edge(1, 3, 4);
    instance.add_edge(1, 4, 5);
    instance.add_edge(1, 5, 6);
    instance.add_edge(2, 3, 3);
    instance.add_edge(2, 4, 5);
    instance.add_edge(2, 5, 6);
    instance.add_edge(3, 4, 4);
    instance.add_edge(3, 5, 6);
    instance.add_edge(4, 5, 5);

    // let instance = InstanceParser::get_instance("").unwrap();

    let instance = Rc::new(instance);
    let mut greedy = GreedySolver::new(1);
    let logger = Logger::new(&greedy, "");
    greedy.solve(Rc::clone(&instance), logger);

    println!("{:?}", greedy.current_solution());

    greedy.current_solution_mut().calculate_objective_value();
    println!("{:?}", greedy.current_solution().driver_distances());
    println!("Before: {}", greedy.current_solution().objective_value());
    let start = 1;
    let length = 2;
    let length_2 = 1;
    println!("Delta: {}", TripleEdgeExchange::get_delta(greedy.current_solution(), start, length, length_2));
    let new_val = TripleEdgeExchange::get_delta(&greedy.current_solution(), start, length, length_2) + greedy.current_solution().objective_value();
    TripleEdgeExchange::apply(greedy.current_solution_mut(), start, length, length_2, true);
    println!("{:?}", greedy.current_solution().driver_distances());
    println!("{:?}", greedy.current_solution());
    println!("{}, {}", new_val, greedy.current_solution().objective_value());
    greedy.current_solution_mut().calculate_objective_value();
    println!("obj {}", greedy.current_solution().objective_value());

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