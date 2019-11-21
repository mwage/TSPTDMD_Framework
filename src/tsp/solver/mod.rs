mod greedy;
mod random_greedy;
mod pilot;

pub use greedy::GreedySolver;
pub use random_greedy::RandomGreedySolver;
pub use pilot::PilotSolver;

use crate::tsp::TSPInstance;
use crate::tsp::io::Logger;

use std::rc::Rc;

pub trait Solver {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger);   // Solve TSP instance
    fn to_string(&self) -> String;    // Used for logging
}