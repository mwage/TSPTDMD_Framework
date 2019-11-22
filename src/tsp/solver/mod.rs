mod greedy;
mod pilot;
mod local_search;

pub use greedy::GreedySolver;
pub use pilot::PilotSolver;
pub use local_search::LocalSearch;

use crate::tsp::TSPInstance;
use crate::tsp::io::Logger;

use std::rc::Rc;

pub trait Solver {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger);   // Solve TSP instance
    fn to_string(&self) -> String;    // Used for logging
}