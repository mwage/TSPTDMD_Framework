use std::rc::Rc;

use crate::tsp::io::Logger;
use crate::tsp::TSPInstance;
use crate::tsp::solver::Solver;
use crate::tsp::neighborhood::NeighborhoodImpl;
use crate::Solution;
use crate::GreedySolver;

pub struct SimulatedAnnealing<N: NeighborhoodImpl> {
    neighborhood: N,
    temperature: f64,
    alpha: f64,
    terminating_temperature: f64
}

impl<N> SimulatedAnnealing<N> where N: NeighborhoodImpl {
    pub fn new(neighborhood: N) -> Self {
        SimulatedAnnealing {
            neighborhood,
            temperature: 4.0,
            alpha: 0.999,
            terminating_temperature: 10f64.powf(-10f64)
        }
    }

    fn decrease_temperature(&self) {

    }

    // fn accept(&self) -> bool {

    //     let x = e.powf(- delta / T)
    //rand 0,1 < x
    // }
}

impl<N> Solver for SimulatedAnnealing<N> where N: NeighborhoodImpl {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        let mut best_solution = Solution::new(Rc::clone(&instance));
        let mut greedy = GreedySolver::new(1);
        greedy.set_instance(&instance);
        greedy.solve_greedy(&mut best_solution, &logger);


    }

    fn to_string(&self) -> String {
        format!("SimulatedAnnealing.{}", self.neighborhood.to_string())
    }
}