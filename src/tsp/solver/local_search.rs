use std::rc::Rc;

use super::Solver;
use crate::tsp::io::Logger;
use crate::Solution;
use crate::TSPInstance;
use crate::tsp::neighborhood::NeighborhoodImpl;
use crate::StepFunction;

pub struct LocalSearch<N: NeighborhoodImpl> {
    neighborhood: N,
    step_function: StepFunction,
    iteration_limit: usize
}

impl<N> LocalSearch<N> where N: NeighborhoodImpl {
    pub fn new(neighborhood: N, step_function: StepFunction, iteration_limit: usize) -> Self {
        LocalSearch {
            neighborhood,
            step_function,
            iteration_limit
        }
    }

    pub fn local_search(&self, solution: &mut Solution) {
        // if self.step_function == StepFunction::Random {
        //     self.search_random(solution)
        // } else {
        //     self.search_deterministic(solution)
        // }
    }
    
    fn search_deterministic(&self, solution: &mut Solution) {
        let mut counter = 0;
        loop {
            let improved = self.neighborhood.get_neighbor(solution, &self.step_function, true);    // TODO: Set delta eval

            if !improved || counter >= self.iteration_limit {
                break;
            }
            counter += 1;
        }
    }

    fn search_random(&self, solution: &mut Solution) {
        let mut counter = 0;
        let mut best_solution = solution.clone();
        loop {
            self.neighborhood.get_neighbor(solution, &self.step_function, true);    // TODO: Set delta eval

            if solution.objective_value() < best_solution.objective_value() {
                best_solution = solution.clone();
            }
            if counter >= self.iteration_limit {
                break;
            }
            counter += 1;
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