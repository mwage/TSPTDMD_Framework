#![allow(dead_code)]

extern crate rand;

mod tsp;

use tsp::TestRunner;
use tsp::solver::GreedySolver;
use tsp::solver::PilotSolver;
use tsp::solver::LocalSearch;
use tsp::neighborhood::NeighborhoodImpl;
use tsp::neighborhood::DoubleEdgeExchange;
use tsp::neighborhood::DriverFlip;
use tsp::neighborhood::TripleEdgeExchange;


use tsp::Solution;
use tsp::TSPInstance;
use std::rc::Rc;

// exports
pub use tsp::neighborhood::Neighborhood;
pub use tsp::neighborhood::StepFunction;


static TIME_LIMIT: u128 = 1000 * 60 * 15;


pub fn pilot(instance_name: Option<&str>, beta: usize) {
    TestRunner::solve_instance(PilotSolver::new(beta), instance_name);
}

pub fn greedy(instance_name: Option<&str>, candidate_size: usize) {
    TestRunner::solve_instance(GreedySolver::new(candidate_size), instance_name);
}

pub fn local_search(neighborhood: Neighborhood, step_function: StepFunction, instance_name: Option<&str>) {
    match neighborhood {
        Neighborhood::DriverFlip => start_local_search(DriverFlip::new(), step_function, instance_name),
        Neighborhood::DoubleEdgeExchange(x) => start_local_search(DoubleEdgeExchange::new(x), step_function, instance_name),
        Neighborhood::TripleEdgeExchange(x) => start_local_search(TripleEdgeExchange::new(x), step_function, instance_name),
        _ => unimplemented!()
    };
}

fn start_local_search<N> (neighborhood: N, step_function: StepFunction, instance_name: Option<&str>) where N: NeighborhoodImpl {
    TestRunner::solve_instance(LocalSearch::new(neighborhood, step_function), instance_name);
}

pub fn grasp(neighborhood: Neighborhood, stepFunction: StepFunction) {

}

pub fn vnd() {

}

pub fn metaheuristic() {

}

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
