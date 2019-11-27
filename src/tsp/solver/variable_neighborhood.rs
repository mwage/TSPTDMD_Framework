use std::rc::Rc;

use crate::tsp::io::Logger;
use crate::tsp::TSPInstance;
use crate::tsp::solver::Solver;
use crate::tsp::neighborhood::NeighborhoodImpl;
use crate::StepFunction;
use crate::Solution;
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
        let mut best_solution = Solution::new(Rc::clone(&instance));
        let mut greedy = GreedySolver::new(1);
        greedy.set_instance(&instance);
        greedy.solve_greedy(&mut best_solution, &logger);

        let mut counter = 0;
        while counter < self.neighborhoods.len() {
            let neighborhood = &mut self.neighborhoods[counter];
            if neighborhood.get_best_improving_neighbor(&best_solution, true) {
                neighborhood.set_neighbor(&mut best_solution, true);
                counter = 0;
            }
            counter += 1;
            // TODO Time constraint
        }

        logger.log_result(&best_solution);
    }

    fn to_string(&self) -> String {
        String::from("SimulatedAnnealing")
    }
}