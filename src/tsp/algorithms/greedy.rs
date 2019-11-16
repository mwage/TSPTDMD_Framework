use super::Solver;
use crate::tsp::TSPInstance;
use crate::tsp::Logger;

pub struct GreedySolver {

}

impl GreedySolver {
    pub fn new() -> GreedySolver {
        GreedySolver{}
    }
}

impl Solver for GreedySolver {
    fn solve(&mut self, instance: &mut TSPInstance, logger: &Logger) {
        // TODO
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
    fn solve(&mut self, instance: &mut TSPInstance, logger: &Logger) {
        // TODO
    }
}