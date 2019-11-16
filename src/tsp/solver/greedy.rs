use super::Solver;
use crate::tsp::io::Logger;
use crate::tsp::TSPInstance;

pub struct GreedySolver {

}

impl GreedySolver {
    pub fn new() -> GreedySolver {
        GreedySolver{}
    }
}

impl Solver for GreedySolver {
    fn solve(&mut self, instance: TSPInstance, logger: Logger) {
        // TODO
    }

    fn to_string(&self) -> &str {
        "Greedy"
    }
}

pub struct RandomGreedySolver {

}

impl RandomGreedySolver {
    pub fn new() -> GreedySolver {
        GreedySolver{}
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