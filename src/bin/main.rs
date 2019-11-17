extern crate tsp;

use tsp::Neighborhood;
use tsp::StepFunction;

fn main() {
    tsp::deterministic_construction_heuristic(Some("0010_k1.txt"));
    // tsp::local_search(Neighborhood::Flip, StepFunction::BestImprovement);
    // test_all_local_searches();
}

fn test_all_local_searches() {
    let neighborhoods = vec![Neighborhood::Flip];
    let step_functions = vec![StepFunction::BestImprovement];
    for neighborhood in neighborhoods.iter() {
        for stepfunction in step_functions.iter() {
            tsp::local_search(neighborhood.to_owned(), stepfunction.to_owned());
        }
    }
}