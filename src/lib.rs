#![allow(dead_code)]

mod tsp;
use tsp::TestRunner;
use tsp::solver::Solver;
use tsp::solver::GreedySolver;
use tsp::neighborhood::NeighborhoodImpl;
use tsp::step_function::StepFunctionImpl;

// exports
pub use tsp::neighborhood::Neighborhood;
pub use tsp::step_function::StepFunction;



pub fn deterministic_construction_heuristic(instance_name: Option<&str>) {
    TestRunner::solve_instance(GreedySolver::new(), instance_name);
}

pub fn randomized_construction_heuristic() {

}

pub fn local_search(neighborhoodFunction: Neighborhood, stepFunction: StepFunction) {
    // let neighborhoodImpl = selectNeighborhood(neighborhoodFunction);
    // let stepFunctionImpl = selectStepFunction(stepFunction);
}

pub fn grasp(neighborhoodFunction: Neighborhood, stepFunction: StepFunction) {

}

pub fn vnd() {

}

pub fn metaheuristic() {

}

// fn selectNeighborhood(neighborhood: Neighborhood) -> impl NeighborhoodImpl {
//     unimplemented!();
// }

// fn selectStepFunction(stepFunction: StepFunction) -> impl StepFunctionImpl {
//     unimplemented!();
// }