#![allow(dead_code)]

extern crate tsp_framework;

use std::env;

use tsp_framework::Neighborhood;
use tsp_framework::StepFunction;
use tsp_framework::greedy;
use tsp_framework::pilot;
use tsp_framework::local_search;
use tsp_framework::grasp;
use tsp_framework::variable_neighborhood;


fn main() {
    all_from_env();
    // greedy(Some("berlin52_k2_2"), 1, 1);
    // pilot(Some("berlin52_k4_1"), 15, 1);
    // pilot(Some("0010_k1"), 100);
    // greedy(Some("berlin52_k2_2"), 3);
    // local_search(Some("berlin52_k2_2"), Neighborhood::DoubleEdgeExchange(5), StepFunction::BestImprovement, 10000, 1);
    
    // test_all_local_searches();
    // variable_neighborhood(Some("berlin52_k2_2"), vec![Neighborhood::DoubleEdgeExchange(None), Neighborhood::DriverFlip, Neighborhood::TripleEdgeExchange(None)], 1);
    // grasp(Some("berlin52_k2_2"), 5, Neighborhood::DoubleEdgeExchange(None), StepFunction::BestImprovement, 100, 1000, 1);
}

fn all_from_env( ) {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);
    match &args[1][..] {
        "greedy" => greedy(None, args[2].parse::<usize>().unwrap(), 1),
        "pilot" => pilot(None, args[2].parse::<usize>().unwrap(), 1),
        "local" => test_all_local_searches(None),
        "grasp" => grasp(None, 5, Neighborhood::DoubleEdgeExchange(None), StepFunction::BestImprovement, 1000, 10000, 1),
        "vnd" => variable_neighborhood(None, vec![Neighborhood::DoubleEdgeExchange(None), 
            Neighborhood::DriverFlip, Neighborhood::TripleEdgeExchange(None)], 1),
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