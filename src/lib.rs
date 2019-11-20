#![allow(dead_code)]

extern crate rand;

mod tsp;

use tsp::TestRunner;
use tsp::solver::GreedySolver;
use tsp::solver::RandomGreedySolver;
use tsp::solver::PilotSolver;
use tsp::neighborhood::NeighborhoodImpl;
use tsp::neighborhood::Flip;
use tsp::step_function::StepFunctionImpl;
use tsp::step_function::BestImprovement;

// exports
pub use tsp::neighborhood::Neighborhood;
pub use tsp::step_function::StepFunction;

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
        Neighborhood::Flip => Flip::new(),
        _ => unimplemented!()
    }
}

fn selectStepFunction(stepFunction: StepFunction) -> impl StepFunctionImpl {
    match stepFunction {
        StepFunction::BestImprovement => BestImprovement::new(),
        _ => unimplemented!()
    }
}