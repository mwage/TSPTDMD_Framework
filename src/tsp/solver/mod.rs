mod grasp;
mod greedy;
mod local_search;
mod pilot;
mod simulated_annealing;
mod variable_neighborhood;

pub use greedy::GreedySolver;
pub use grasp::Grasp;
pub use local_search::LocalSearch;
pub use pilot::PilotSolver;
pub use simulated_annealing::SimulatedAnnealing;
pub use variable_neighborhood::VariableNeighborhood;

use crate::tsp::TSPInstance;
use crate::tsp::io::Logger;

use std::rc::Rc;

pub trait Solver {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger);   // Solve TSP instance
    fn to_string(&self) -> String;    // Used for logging
}