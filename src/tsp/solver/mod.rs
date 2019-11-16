mod greedy;

pub use greedy::GreedySolver;
pub use greedy::RandomGreedySolver;

use crate::tsp::TSPInstance;
use crate::tsp::io::Logger;

pub trait Solver {
    fn solve(&mut self, instance: &mut TSPInstance, logger: &Logger);   // Solve TSP instance
    fn to_string(&self) -> &str;    // Used for logging
}