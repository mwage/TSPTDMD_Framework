extern crate tsp_framework;

use tsp_framework::Neighborhood;
use tsp_framework::StepFunction;
use tsp_framework::greedy;
use tsp_framework::randomized_construction_heuristic;
use tsp_framework::pilot;
use tsp_framework::local_search;

fn main() {
    // greedy(Some("berlin52_k2_2"));
    // pilot(Some("berlin52_k2_2"), 15);
    randomized_construction_heuristic(Some("berlin52_k2_2"), 3);

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