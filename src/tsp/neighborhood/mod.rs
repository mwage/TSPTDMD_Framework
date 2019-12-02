mod compound;
mod driver_flip;
mod double_edge_exchange;
mod triple_edge_exchange;

pub use compound::Compound;
pub use driver_flip::DriverFlip;
pub use double_edge_exchange::DoubleEdgeExchange;
pub use triple_edge_exchange::TripleEdgeExchange;

use std::fmt;

use crate::tsp::Solution;
use crate::tsp::io::Logger;

// TODO: Implement non delta eval

pub trait NeighborhoodImpl {
    fn get_random_neighbor(&mut self, solution: &Solution) -> bool;
    fn get_best_improving_neighbor(&mut self, solution: &Solution, logger: &Logger) -> bool;
    fn get_first_improving_neighbor(&mut self, solution: &Solution, logger: &Logger) -> bool;
    fn set_neighbor(&mut self, solution: &mut Solution);
    fn delta(&self) -> Option<isize>;
    fn to_string(&self) -> String;    // Used for logging

    fn get_neighbor(&mut self, solution: &mut Solution, step_function: &StepFunction, logger: &Logger) -> bool {  // Match stepfunction
        match step_function {
            StepFunction::Random => self.get_random_neighbor(solution),
            StepFunction::BestImprovement => self.get_best_improving_neighbor(solution, logger),
            StepFunction::FirstImprovement => self.get_first_improving_neighbor(solution, logger),
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Neighborhood {
    DoubleEdgeExchange(Option<usize>),
    DriverFlip,
    TripleEdgeExchange(Option<usize>),
    Compound(Option<usize>)
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