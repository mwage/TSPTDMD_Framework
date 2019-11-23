use std::rc::Rc;

use super::Solver;
use crate::tsp::io::Logger;
use crate::Solution;
use crate::TSPInstance;
use crate::tsp::neighborhood::NeighborhoodImpl;
use crate::StepFunction;

pub struct LocalSearch<N: NeighborhoodImpl> {
    neighborhood: N,
    step_function: StepFunction
}

impl<N> LocalSearch<N> where N: NeighborhoodImpl {
    pub fn new(neighborhood: N, step_function: StepFunction) -> Self {
        LocalSearch {
            neighborhood,
            step_function
        }
    }

    pub fn neighborhood_to_string(&self) -> String {
        format!("{}.{}", self.step_function, self.neighborhood_to_string())
    }

    fn get_neighbor(&mut self, solution: &mut Solution) {
        self.neighborhood.get_neighbor(solution, &self.step_function, true);
    }
}

impl<N> Solver for LocalSearch<N> where N: NeighborhoodImpl {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        let mut solution = Solution::new_random(instance.clone());
        self.get_neighbor(&mut solution);
    }

    fn to_string(&self) -> String {
        format!("LocalSearch.{}", self.neighborhood_to_string())
    }
}