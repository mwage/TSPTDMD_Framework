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
    // all_from_env();
    // greedy(Some("berlin52_k2_2"), 1, 1);
    // pilot(Some("a280_k1_1"), 1, 1);
    // pilot(Some("0010_k1"), 100);
    // greedy(Some("berlin52_k2_2"), 3);
    local_search(None, Neighborhood::Compound(Some(10)), StepFunction::FirstImprovement, 10000, 1);
    
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
        "local" => {
            let max_length = if args.len() > 2 {
                Some(args[2].parse::<usize>().unwrap())
            } else {
                None
            };
            test_all_local_searches(None, max_length)
        },   // TODO: set blocklength
        "grasp" => {
            let max_length = if args.len() > 2 {
                Some(args[2].parse::<usize>().unwrap())
            } else {
                None
            };
            grasp(None, 5, Neighborhood::DoubleEdgeExchange(max_length), StepFunction::BestImprovement, 100, 20000, 1)
        },   // TODO: set blocklength
        "vnd" => {
            let max_length = if args.len() > 2 {
                Some(args[2].parse::<usize>().unwrap())
            } else {
                None
            };
            variable_neighborhood(None, vec![Neighborhood::DoubleEdgeExchange(max_length),
                Neighborhood::DriverFlip, Neighborhood::TripleEdgeExchange(max_length)], 1)
        },
        _ => unimplemented!()
    };
}

fn test_all_local_searches(instance: Option<&str>, max_length: Option<usize> ) {
    let neighborhoods = vec![Neighborhood::DoubleEdgeExchange(max_length), Neighborhood::TripleEdgeExchange(max_length), Neighborhood::DriverFlip];
    let step_functions = vec![StepFunction::BestImprovement, StepFunction::FirstImprovement, StepFunction::Random];
    for neighborhood in neighborhoods.iter() {
        for step_function in step_functions.iter() {
            let runs = match step_function {
                StepFunction::Random => 5,
                _ => 1
            };
            local_search(instance, neighborhood.to_owned(), step_function.to_owned(), 20000, runs);
        }
    }
}