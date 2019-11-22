mod driver_flip;
mod driver_swap;
mod double_edge_exchange;
mod triple_edge_exchange;

pub use driver_flip::DriverFlip;
pub use driver_swap::DriverSwap;
pub use double_edge_exchange::DoubleEdgeExchange;
pub use triple_edge_exchange::TripleEdgeExchange;



use std::fmt;

use crate::tsp::Solution;

pub trait NeighborhoodImpl {
    fn get_random_neighbor(&self, solution: &mut Solution);
    fn get_best_improving_neighbor(&self, solution: &mut Solution);
    fn to_string(&self) -> String;    // Used for logging
}

#[derive(Debug, Clone)]
pub enum Neighborhood {
    DoubleEdgeExchange(usize),
    DriverFlip,
    TripleEdgeExchange(usize),
    DriverSwap
}

#[derive(Debug, Clone)]
pub enum StepFunction {
    Random,
    BestImprovement
}

impl fmt::Display for StepFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}