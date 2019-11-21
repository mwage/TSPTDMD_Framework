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

    fn get_neighbor(&self, solution: &mut Solution) {
        match self.step_function {
            StepFunction::Random => self.neighborhood.get_random_neighbor(solution),
            StepFunction::BestImprovement => self.neighborhood.get_best_improving_neighbor(solution),
            _ => unimplemented!()
        }
    }
}

impl<N> Solver for LocalSearch<N> where N: NeighborhoodImpl {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        let mut solution = Solution::new_random(instance.clone());
        self.get_neighbor(&mut solution);
    }

    fn to_string(&self) -> String {
        format!("LocalSearch.{}.{}", self.step_function, self.neighborhood.to_string())
    }
}