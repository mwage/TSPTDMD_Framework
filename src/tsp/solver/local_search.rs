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

    pub fn local_search(&mut self, solution: Solution, logger: &Logger) -> Solution {
        match self.step_function {
            StepFunction::Random => self.search_random(solution, logger),
            _ => self.search_deterministic(solution, logger)
        }
    }
    
    fn search_deterministic(&mut self, mut solution: Solution, logger: &Logger) -> Solution {
        let mut counter = 0;
        loop {
            let improved = self.neighborhood.get_neighbor(&mut solution, &self.step_function, true);    // TODO: Set delta eval

            if !improved || counter >= self.iteration_limit || logger.get_elapsed() >= crate::TIME_LIMIT {
                break;
            }
            
            self.neighborhood.set_neighbor(&mut solution, true);
            counter += 1;
        }

        solution
    }

    fn search_random(&mut self, mut solution: Solution, logger: &Logger) -> Solution {
        let mut counter = 0;
        loop {
            self.neighborhood.get_neighbor(&mut solution, &self.step_function, true);    // TODO: Set delta eval    

            if self.neighborhood.delta().unwrap() < 0 {
                self.neighborhood.set_neighbor(&mut solution, true);
            }
            if counter >= self.iteration_limit || logger.get_elapsed() >= crate::TIME_LIMIT {
                break;
            }

            counter += 1;
        }

        solution
    }

    pub fn neighborhood_to_string(&self) -> String {
        format!("{}.{}", self.step_function, self.neighborhood.to_string())
    }
}

impl<N> Solver for LocalSearch<N> where N: NeighborhoodImpl {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        let mut solution = Solution::new(Rc::clone(&instance));
        let mut greedy = GreedySolver::new(1);
        greedy.set_instance(&instance);
        greedy.solve_greedy(&mut solution, &logger);
        solution.calculate_objective_value();
        println!("{}", solution.objective_value());
        let solution = self.local_search(solution, &logger);
        println!("{}", solution.objective_value());
        logger.log_result(&solution);
    }

    fn to_string(&self) -> String {
        format!("ls.{}", self.neighborhood_to_string())
    }
}