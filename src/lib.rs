#![allow(dead_code)]

extern crate rand;

mod tsp;

use tsp::TestRunner;
use tsp::solver::GreedySolver;
use tsp::solver::RandomGreedySolver;
use tsp::solver::PilotSolver;
use tsp::neighborhood::NeighborhoodImpl;
use tsp::neighborhood::DriverFlip;
use tsp::step_function::StepFunctionImpl;
use tsp::step_function::BestImprovement;
use tsp::neighborhood::TripleEdgeExchange;


use tsp::Solution;
use tsp::neighborhood::DoubleEdgeExchange;
use tsp::TSPInstance;
use std::rc::Rc;
use crate::rand::Rng;
use rand::prelude::*;

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
        Neighborhood::DriverFlip => DriverFlip::new(),
        _ => unimplemented!()
    }
}

fn selectStepFunction(stepFunction: StepFunction) -> impl StepFunctionImpl {
    match stepFunction {
        StepFunction::BestImprovement => BestImprovement::new(),
        _ => unimplemented!()
    }
}

pub fn test_delta() {
    let vertices = 10;
    let mut instance =TSPInstance::new(vertices, vertices, 100);
    instance.add_edge(0, 1, 10);
    instance.add_edge(0, 2, 5);
    instance.add_edge(0, 3, 106);
    instance.add_edge(0, 4, 52);
    instance.add_edge(1, 2, 24);
    instance.add_edge(1, 3, 17);
    instance.add_edge(1, 4, 20);
    instance.add_edge(2, 3, 17);
    instance.add_edge(2, 4, 20);
    instance.add_edge(3, 4, 47);
    instance.add_edge(0, 5, 20);
    instance.add_edge(1, 5, 20);
    instance.add_edge(2, 5, 17);
    instance.add_edge(3, 5, 20);
    instance.add_edge(4, 5, 47);
    instance.add_edge(0, 6, 20);
    instance.add_edge(1, 6, 20);
    instance.add_edge(2, 6, 17);
    instance.add_edge(3, 6, 20);
    instance.add_edge(4, 6, 47);
    instance.add_edge(5, 6, 47);
    instance.add_edge(0, 7, 20);
    instance.add_edge(1, 7, 20);
    instance.add_edge(2, 7, 17);
    instance.add_edge(3, 7, 20);
    instance.add_edge(4, 7, 47);
    instance.add_edge(5, 7, 47);
    instance.add_edge(6, 7, 47);
    instance.add_edge(0, 8, 20);
    instance.add_edge(1, 8, 20);
    instance.add_edge(2, 8, 17);
    instance.add_edge(3, 8, 20);
    instance.add_edge(4, 8, 47);
    instance.add_edge(5, 8, 47);
    instance.add_edge(6, 8, 47);
    instance.add_edge(7, 8, 47);
    instance.add_edge(0, 9, 20);
    instance.add_edge(1, 9, 20);
    instance.add_edge(2, 9, 17);
    instance.add_edge(3, 9, 20);
    instance.add_edge(4, 9, 47);
    instance.add_edge(5, 9, 47);
    instance.add_edge(6, 9, 47);
    instance.add_edge(7, 9, 47);
    instance.add_edge(8, 9, 47);

    
    let instance = Rc::new(instance);

    let mut solution = Solution::new(Rc::clone(&instance));
    // for i in 0..10 {
    //     solution.add_assignment(i, i, 10);
    // }
    let mut rng = rand::thread_rng();
    let mut vertices: Vec<u32> = (1..vertices as u32).collect();
    vertices.shuffle(&mut rng);
    for vertex in vertices.iter() {
        let driver = rand::thread_rng().gen_range(0, instance.number_of_drivers() as u32);
        let last_vertex = solution.get_last_vertex();
        solution.add_assignment(*vertex, driver, instance.get_vertex(*vertex).get_weight(last_vertex));
    }
    let driver = rand::thread_rng().gen_range(0, instance.number_of_drivers() as u32);
    let last_vertex = solution.get_last_vertex();
    solution.add_assignment(0, driver, instance.get_vertex(0).get_weight(last_vertex));


    solution.calculate_objective_value();
    println!("Before: {}", solution.objective_value());
    // DoubleEdgeExchange::apply(&mut solution, 1, 2, true);
    // DriverFlip::apply(&mut solution, 2, 3, true);
    // println!("{:?}", solution.assignments());
    TripleEdgeExchange::apply(&mut solution, 4, 3, 3, true);
    // println!("{:?}", solution.assignments());
    let x = solution.objective_value();
    solution.calculate_objective_value();
    println!("With Delta: {}", x);
    println!("From distances: {}", solution.objective_value());
}
