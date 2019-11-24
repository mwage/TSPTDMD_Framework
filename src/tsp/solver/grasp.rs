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
    local_search: LocalSearch<N>,
    iteration_limit: usize
}

impl<N> Grasp<N> where N: NeighborhoodImpl {
    pub fn new(neighborhood: N, step_function: StepFunction, candidate_size: usize, iteration_limit: usize, ls_iteration_limit: usize) -> Self {
        assert!(candidate_size > 1);
        Grasp {
            greedy: GreedySolver::new(candidate_size),
            local_search: LocalSearch::new(neighborhood, step_function, ls_iteration_limit),
            iteration_limit
        }
    }    
}

impl<N> Solver for Grasp<N> where N: NeighborhoodImpl {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        let mut best_solution: Solution;
        loop {
            let mut candidate = Solution::new(Rc::clone(&instance));            
            self.greedy.solve_greedy(&mut candidate, &logger);
            
        }
    }

    fn to_string(&self) -> String {
        format!("Grasp.{}.{}", self.local_search.neighborhood_to_string(), self.greedy.candidate_size())
    }
}