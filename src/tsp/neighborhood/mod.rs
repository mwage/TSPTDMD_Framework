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

#[derive(Debug, Clone)]
pub enum Neighborhood {
    DriverFlip
}

impl fmt::Display for Neighborhood {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
    
pub trait NeighborhoodImpl {
    fn apply_neighborhood(&self, solution: &mut Solution);
}
