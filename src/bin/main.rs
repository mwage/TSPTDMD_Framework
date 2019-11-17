extern crate tsp_framework;

use tsp_framework::Neighborhood;
use tsp_framework::StepFunction;
use tsp_framework::deterministic_construction_heuristic;
use tsp_framework::local_search;

fn main() {
    deterministic_construction_heuristic(Some("0010_k1.txt"));
    // tsp::local_search(Neighborhood::Flip, StepFunction::BestImprovement);
    // test_all_local_searches();
}

fn test_all_local_searches() {
    let neighborhoods = vec![Neighborhood::Flip];
    let step_functions = vec![StepFunction::BestImprovement];
    for neighborhood in neighborhoods.iter() {
        for stepfunction in step_functions.iter() {
            local_search(neighborhood.to_owned(), stepfunction.to_owned());
        }
    }
}