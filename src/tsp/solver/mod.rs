mod greedy;
mod partial_greedy;
mod pilot;

pub use greedy::GreedySolver;
pub use pilot::Pilot;
pub use partial_greedy::PartialGreedy;

use crate::tsp::TSPInstance;
use crate::tsp::io::Logger;

use std::rc::Rc;

pub trait Solver {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger);   // Solve TSP instance
    fn to_string(&self) -> &str;    // Used for logging
}