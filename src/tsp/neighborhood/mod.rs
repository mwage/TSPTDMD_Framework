mod driver_flip;
mod double_edge_exchange;
mod triple_edge_exchange;

pub use driver_flip::DriverFlip;
pub use double_edge_exchange::DoubleEdgeExchange;
pub use triple_edge_exchange::TripleEdgeExchange;



use std::fmt;

use crate::tsp::Solution;

pub trait NeighborhoodImpl {
    fn get_random_neighbor(&self, solution: &mut Solution, delta_eval: bool) -> bool;
    fn get_best_improving_neighbor(&self, solution: &mut Solution, delta_eval: bool) -> bool;
    fn get_first_improving_neighbor(&self, solution: &mut Solution, delta_eval: bool) -> bool;
    fn to_string(&self) -> String;    // Used for logging

    fn get_neighbor(&self, solution: &mut Solution, step_function: &StepFunction, delta_eval: bool) -> bool {  // Match stepfunction
        match step_function {
            StepFunction::Random => self.get_random_neighbor(solution, delta_eval),
            StepFunction::BestImprovement => self.get_best_improving_neighbor(solution, delta_eval),
            StepFunction::FirstImprovement => self.get_first_improving_neighbor(solution, delta_eval),
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Neighborhood {
    DoubleEdgeExchange(usize),
    DriverFlip,
    TripleEdgeExchange(usize)
}

#[derive(Debug, Clone)]
pub enum StepFunction {
    Random,
    BestImprovement,
    FirstImprovement
}

impl fmt::Display for StepFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}