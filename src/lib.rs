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
    // let instance = Rc::new(TSPInstance::new_random(10, 4, 200, 100)); 
    // let mut solution = Solution::new_random(Rc::clone(&instance));

    let mut instance =  TSPInstance::new(3, 2, 10);
    instance.add_edge(0, 1, 5);
    instance.add_edge(0, 2, 7);
    instance.add_edge(1, 2, 10);

    let instance = Rc::new(instance);
    let mut greedy = GreedySolver::new(1);
    let logger = Logger::new(&greedy, "");
    greedy.solve(Rc::clone(&instance), logger);

    // println!("{:?}", greedy.current_solution());

    greedy.current_solution_mut().calculate_objective_value();
    println!("Before: {}", greedy.current_solution().objective_value());

    let idx = 1;
    let old_driver = greedy.current_solution().get_assignment(idx).driver();
    assert_eq!(old_driver, 1);
    let new_driver = 0;
    // println!("Delta: {}", DriverFlip::get_delta(greedy.current_solution(), idx, new_driver));
    // let new_val = DriverFlip::get_delta(greedy.current_solution() ,idx, new_driver) + greedy.current_solution().objective_value() as isize;

    // DoubleEdgeExchange::apply(&mut solution, 1, 2, true);
    DriverFlip::apply(greedy.current_solution_mut(), idx, new_driver, true);
    // TripleEdgeExchange::apply(&mut solution, 4, 3, 3, true);

    
    let x = greedy.current_solution().objective_value();
    // solution.calculate_objective_value();
    println!("With Delta: {}", x);
    // println!("From distances: {}, {}", solution.objective_value(), new_val);
}

pub fn modulo(number: isize, module: usize) -> usize {
    let module = module as isize;
    let x = number % module;
    if x < 0 {
        (x + module) as usize
    } else {
        x as usize
    }
}