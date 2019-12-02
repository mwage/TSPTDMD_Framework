use std::rc::Rc;

use crate::tsp::io::Logger;
use crate::tsp::TSPInstance;
use crate::tsp::solver::Solver;
use crate::tsp::neighborhood::NeighborhoodImpl;
use crate::tsp::Solution;
use crate::GreedySolver;

pub struct VariableNeighborhood {
    neighborhoods: Vec<Box<dyn NeighborhoodImpl>>
}

impl VariableNeighborhood {
    pub fn new(neighborhoods: Vec<Box<dyn NeighborhoodImpl>>) -> Self {
        VariableNeighborhood {
            neighborhoods
        }
    }
}

impl Solver for VariableNeighborhood {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        let mut solution = Solution::new(Rc::clone(&instance));
        let mut greedy = GreedySolver::new(1);
        greedy.solve_greedy(&instance, &mut solution, &logger);
        solution.calculate_objective_value();
        
        let mut counter = 0;
        while counter < self.neighborhoods.len() {
            let neighborhood = &mut self.neighborhoods[counter];
            if neighborhood.get_best_improving_neighbor(&solution, &logger) {
                neighborhood.set_neighbor(&mut solution);
                counter = 0;
            } else {
                counter += 1;
            }

            if logger.get_elapsed() >= crate::TIME_LIMIT {
                break;
            }
        }

        logger.log_result(&solution);
    }

    fn to_string(&self) -> String {
        let neighborhoods = self.neighborhoods.iter().map(|x| x.to_string()).fold(String::new(), |a, b| a + &b + ".");
        format!("vnd.{}", neighborhoods)
    }
}