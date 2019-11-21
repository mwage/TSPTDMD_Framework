#![allow(dead_code)]

extern crate rand;

mod tsp;

use tsp::TestRunner;
use tsp::solver::GreedySolver;
use tsp::solver::RandomGreedySolver;
use tsp::solver::PilotSolver;
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

pub fn pilot(instance_name: Option<&str>, beta: usize) {
    TestRunner::solve_instance(PilotSolver::new(beta), instance_name);
}

pub fn greedy(instance_name: Option<&str>) {
    TestRunner::solve_instance(GreedySolver::new(), instance_name);
}

pub fn randomized_construction_heuristic(instance_name: Option<&str>, candidate_size: usize) {
    TestRunner::solve_instance(RandomGreedySolver::new(candidate_size), instance_name)
}

pub fn local_search(neighborhood: Neighborhood, stepFunction: StepFunction) {
    // let neighborhoodImpl = selectNeighborhood(neighborhood);
    // let stepFunctionImpl = selectStepFunction(stepFunction);
}

pub fn grasp(neighborhood: Neighborhood, stepFunction: StepFunction) {

}

pub fn vnd() {

}

pub fn metaheuristic() {

}

fn selectNeighborhood(neighborhood: Neighborhood) -> impl NeighborhoodImpl {
    match neighborhood {
        Neighborhood::DriverFlip => DriverFlip::new(),
        _ => unimplemented!()
    }
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
