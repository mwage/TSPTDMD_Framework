#![allow(dead_code)]

extern crate tsp_framework;

use std::env;

use tsp_framework::Neighborhood;
use tsp_framework::StepFunction;
use tsp_framework::greedy;
use tsp_framework::pilot;
use tsp_framework::local_search;
use tsp_framework::test_delta;
use tsp_framework::grasp;


fn main() {
    // all_from_env();
    // greedy(Some("berlin52_k2_2"), 1, 1);
    // pilot(Some("berlin52_k2_2"), 15, 1);
    // pilot(Some("0010_k1"), 100);
    // greedy(Some("berlin52_k2_2"), 3);
    // test_delta();
    local_search(Some("berlin52_k2_2"), Neighborhood::DoubleEdgeExchange(5), StepFunction::BestImprovement, 10000, 1);
    // test_all_local_searches();
    
    // grasp(Some("berlin52_k2_2"), 5, Neighborhood::DriverFlip, StepFunction::BestImprovement, 1000, 100, 1);
}

fn all_from_env( ) {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 3);
    match &args[1][..] {
        "greedy" => greedy(None, args[2].parse::<usize>().unwrap(), 1),
        "pilot" => pilot(None, args[2].parse::<usize>().unwrap(), 1),
        _ => unimplemented!()
    };
}

fn test_all_local_searches(instance: Option<&str>) {
    let neighborhoods = vec![Neighborhood::DriverFlip];
    let step_functions = vec![StepFunction::BestImprovement];
    for neighborhood in neighborhoods.iter() {
        for stepfunction in step_functions.iter() {
            local_search(instance, neighborhood.to_owned(), stepfunction.to_owned(), 10000, 1);
        }
    }
}