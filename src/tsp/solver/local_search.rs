use std::rc::Rc;

use super::Solver;
use crate::tsp::io::Logger;
use crate::tsp::Solution;
use crate::tsp::TSPInstance;
use crate::tsp::neighborhood::NeighborhoodImpl;
use crate::StepFunction;
use crate::GreedySolver;

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

    pub fn local_search(&mut self, solution: &mut Solution, logger: &Logger) {
        match self.step_function {
            StepFunction::Random => self.search_random(solution, logger),
            _ => self.search_deterministic(solution, logger)
        };
    }
    
    fn search_deterministic(&mut self, solution: &mut Solution, logger: &Logger) {
        let mut counter = 0;
        loop {
            let improved = self.neighborhood.get_neighbor(solution, &self.step_function, false, logger);    // TODO: Set delta eval

            if !improved || counter >= self.iteration_limit || logger.get_elapsed() >= crate::TIME_LIMIT {
                break;
            }
            
            self.neighborhood.set_neighbor(solution, true);
            counter += 1;
        }
    }

    fn search_random(&mut self, solution: &mut Solution, logger: &Logger) {
        let mut counter = 0;
        loop {
            self.neighborhood.get_neighbor(solution, &self.step_function, true, logger);    // TODO: Set delta eval    

            if self.neighborhood.delta().unwrap() < 0 {
                self.neighborhood.set_neighbor(solution, true);
            }
            if counter >= self.iteration_limit || logger.get_elapsed() >= crate::TIME_LIMIT {
                break;
            }

            counter += 1;
        }
    }

    pub fn neighborhood_to_string(&self) -> String {
        format!("{}.{}", self.step_function, self.neighborhood.to_string())
    }
}

impl<N> Solver for LocalSearch<N> where N: NeighborhoodImpl {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        let mut solution = Solution::new(Rc::clone(&instance));
        let mut greedy = GreedySolver::new(1);
        greedy.solve_greedy(&instance, &mut solution, &logger);
        solution.calculate_objective_value();
        self.local_search(&mut solution, &logger);
        logger.log_result(&solution);
    }

    fn to_string(&self) -> String {
        format!("ls.{}.{}", self.iteration_limit, self.neighborhood_to_string())
    }
}