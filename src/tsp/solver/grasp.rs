use std::rc::Rc;

use super::GreedySolver;
use super::LocalSearch;
use super::Solver;
use crate::tsp::io::Logger;
use crate::StepFunction;
use crate::tsp::neighborhood::NeighborhoodImpl;
use crate::tsp::TSPInstance;
use crate::tsp::Solution;

pub struct Grasp<N: NeighborhoodImpl> {
    greedy: GreedySolver,
    local_search: LocalSearch<N>,
    iteration_limit: usize,
    base: f64,
    max_beta: usize
}

impl<N> Grasp<N> where N: NeighborhoodImpl {
    pub fn new(neighborhood: N, step_function: StepFunction, max_beta: usize, iteration_limit: usize, ls_iteration_limit: usize) -> Self {        
        assert!(max_beta > 1);        
        let base = 2_f64.powf((iteration_limit as f64).log2() / (max_beta - 1) as f64);

        Grasp {
            greedy: GreedySolver::new(1),
            local_search: LocalSearch::new(neighborhood, step_function, ls_iteration_limit),
            iteration_limit,
            base,
            max_beta
        }
    }    
}

impl<N> Solver for Grasp<N> where N: NeighborhoodImpl {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        let mut best_solution = Solution::new(Rc::clone(&instance));
        let mut beta = 1;
        let mut next_beta_increment = 1;
        let mut counter = 0;

        loop {
            let mut candidate = Solution::new(Rc::clone(&instance));
            self.greedy.solve_greedy(&instance, &mut candidate, &logger);
            candidate.calculate_objective_value();
            self.local_search.local_search(&mut candidate, &logger);

            if candidate.objective_value() < best_solution.objective_value() {
                best_solution = candidate;
            }

            counter += 1;
            if counter >= next_beta_increment {
                next_beta_increment = (next_beta_increment as f64 * self.base).ceil() as usize;
                beta += 1;
                self.greedy = GreedySolver::new(beta);
            }

            if counter > self.iteration_limit || logger.get_elapsed() >= crate::TIME_LIMIT {
                break;
            }
        }

        logger.log_result(&mut best_solution);
    }

    fn to_string(&self) -> String {
        format!("grasp.{}.{}.{}", self.max_beta, self.iteration_limit, self.local_search.to_string())
    }
}