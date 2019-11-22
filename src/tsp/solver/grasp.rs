use std::rc::Rc;

use super::GreedySolver;
use super::LocalSearch;
use super::Solver;
use crate::tsp::io::Logger;
use crate::StepFunction;
use crate::tsp::neighborhood::NeighborhoodImpl;
use crate::TSPInstance;
use crate::Solution;

pub struct Grasp<N: NeighborhoodImpl> {
    greedy: GreedySolver,
    local_search: LocalSearch<N>
}

impl<N> Grasp<N> where N: NeighborhoodImpl {
    pub fn new(neighborhood: N, step_function: StepFunction, candidate_size: usize) -> Self {
        assert!(candidate_size > 1);
        Grasp {
            greedy: GreedySolver::new(candidate_size),
            local_search: LocalSearch::new(neighborhood, step_function)
        }
    }    
}

impl<N> Solver for Grasp<N> where N: NeighborhoodImpl {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        let mut solution = Solution::new_random(instance.clone());
    }

    fn to_string(&self) -> String {
        format!("Grasp.{}.{}", self.local_search.neighborhood_to_string(), self.greedy.candidate_size())
    }
}