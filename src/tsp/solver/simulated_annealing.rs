use std::rc::Rc;
use std::f64::consts::E;

use crate::tsp::io::Logger;
use crate::tsp::TSPInstance;
use crate::tsp::solver::Solver;
use crate::tsp::neighborhood::NeighborhoodImpl;
use crate::tsp::Solution;
use crate::GreedySolver;
use crate::rand::Rng;

pub struct SimulatedAnnealing<N: NeighborhoodImpl> {
    neighborhood: N,
    temperature: f64,
    alpha: f64,
    starting_temperature: f64,
    terminating_temperature: f64
}

// TODO: Implement SA
impl<N> SimulatedAnnealing<N> where N: NeighborhoodImpl {
    pub fn new(neighborhood: N) -> Self {
        SimulatedAnnealing {
            neighborhood,
            temperature: 4.0,
            alpha: 0.99999,
            starting_temperature: 4.0,
            terminating_temperature: 10f64.powf(-10f64)
        }
    }

    fn decrease_temperature(&mut self) {
        self.temperature *= self.alpha;
    }

    fn accept(&self, delta: isize) -> bool {
        if delta < 0 {
            return true;
        }

        let x = E.powf(- delta as f64 / self.temperature);
        let mut rng = rand::thread_rng();
        rng.gen::<f64>() < x
    }
}

impl<N> Solver for SimulatedAnnealing<N> where N: NeighborhoodImpl {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        let mut best_solution = Solution::new(Rc::clone(&instance));
        self.temperature = self.starting_temperature;
        let mut greedy = GreedySolver::new(1);
        greedy.solve_greedy(&instance, &mut best_solution, &logger);
        best_solution.calculate_objective_value();
        
        while self.temperature > self.terminating_temperature {

            if !self.neighborhood.get_random_neighbor(&best_solution) {
                continue;
            }
            if self.accept(self.neighborhood.delta().unwrap()) {
                self.neighborhood.set_neighbor(&mut best_solution);
            }

            if logger.get_elapsed() >= crate::TIME_LIMIT {
                break;
            }

            self.decrease_temperature();
        }
        logger.log_result(&mut best_solution);
    }

    fn to_string(&self) -> String {
        format!("sa.{}", self.neighborhood.to_string())
    }
}