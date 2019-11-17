use super::Solver;

use crate::tsp::io::Logger;
use crate::tsp::TSPInstance;
use crate::tsp::Solution;


pub struct GreedySolver {
    current_solution: Solution
}

impl GreedySolver {
    pub fn new() -> GreedySolver {
        GreedySolver{
            current_solution: Solution::new()
        }
    }
}

impl Solver for GreedySolver {
    fn solve(&mut self, instance: TSPInstance, logger: Logger) {
        // TODO
        logger.log_result(&self.current_solution);
    }

    fn to_string(&self) -> &str {
        "Greedy"
    }
}

pub struct RandomGreedySolver {
    current_solution: Solution
}

impl RandomGreedySolver {
    pub fn new() -> RandomGreedySolver {
        RandomGreedySolver{
            current_solution: Solution::new()
        }
    }
}

impl Solver for RandomGreedySolver {
    fn solve(&mut self, instance: TSPInstance, logger: Logger) {
        // TODO
    }
    
    fn to_string(&self) -> &str {
        "Random Greedy"
    }
}