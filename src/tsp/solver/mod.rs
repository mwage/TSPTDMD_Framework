mod greedy;

pub use greedy::GreedySolver;

use crate::tsp::TSPInstance;
use crate::tsp::io::Logger;

pub trait Solver {
    fn solve(&mut self, instance: TSPInstance, logger: Logger);   // Solve TSP instance
    fn to_string(&self) -> &str;    // Used for logging
}