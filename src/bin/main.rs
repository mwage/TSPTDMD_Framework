extern crate tsp_framework;

use tsp_framework::Neighborhood;
use tsp_framework::StepFunction;
use tsp_framework::greedy;
use tsp_framework::randomized_construction_heuristic;
use tsp_framework::pilot;
use tsp_framework::local_search;
use tsp_framework::test_delta;

fn main() {
    // greedy(Some("3000_k2"));
    // pilot(Some("berlin52_k2_2"), 15);
    pilot(Some("3000_k2"), 1);
    // randomized_construction_heuristic(Some("berlin52_k2_2"), 3);
    // test_delta();
    // local_search(Neighborhood::DoubleEdgeExchange, StepFunction::BestImprovement, Some("berlin52_k2_2"));
    // test_all_local_searches();
}

fn test_all_local_searches(instance: Option<&str>) {
    let neighborhoods = vec![Neighborhood::DriverFlip];
    let step_functions = vec![StepFunction::BestImprovement];
    for neighborhood in neighborhoods.iter() {
        for stepfunction in step_functions.iter() {
            local_search(neighborhood.to_owned(), stepfunction.to_owned(), instance);
        }
    }
}