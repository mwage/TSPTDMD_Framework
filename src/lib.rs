#![allow(dead_code)]

mod tsp;
use tsp::TestRunner;

// exports
pub use tsp::algorithms::Algorithm;
pub use tsp::algorithms::NeighborhoodFunction;
pub use tsp::algorithms::StepFunction;
pub use tsp::algorithms::Neighborhood;
pub use tsp::algorithms::StepFunctionImpl;


pub fn deterministicConstructionHeuristic() {

}

pub fn randomizedConstructionHeuristic() {

}

pub fn localSearch(neighborhoodFunction: NeighborhoodFunction, stepFunction: StepFunction) {
    
}

pub fn testAllLocalSearches() {
    let neighborhoods = vec![NeighborhoodFunction::Flip];
    let stepFunctions = vec![StepFunction::BestImprovement];
    for neighborhood in neighborhoods.iter().map(|x| selectNeighborhood(x)) {
        for stepfunction in stepFunctions.iter().map(|x| selectStepFunction(x)) {
        
        }
    }

}

pub fn grasp(neighborhoodFunction: NeighborhoodFunction, stepFunction: StepFunction) {

}

pub fn vnd() {

}

pub fn metaheuristic() {

}

fn selectNeighborhood(neighborhood: &NeighborhoodFunction) -> impl Neighborhood {
    unimplemented!()
}

fn selectStepFunction(stepFunction: &StepFunction) -> impl StepFunctionImpl {
    unimplemented!()
}



pub fn test_instance(instance: &str, algorithm: Algorithm) {
    TestRunner::run_instance(&algorithm, instance);
}



pub fn test_all_instances(algorithm: Algorithm, neighborhood: Option<Vec<NeighborhoodFunction>>) {
    TestRunner::
    TestRunner::run_all_instances(algorithm, neighborhood);
}