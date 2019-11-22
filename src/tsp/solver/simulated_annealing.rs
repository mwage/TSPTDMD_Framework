use std::rc::Rc;

use crate::tsp::io::Logger;
use crate::tsp::TSPInstance;
use crate::tsp::solver::Solver;
use crate::tsp::neighborhood::NeighborhoodImpl;
use crate::StepFunction;

pub struct SimulatedAnnealing {
    neighborhoods: Vec<Box<dyn NeighborhoodImpl>>,
    step_functions: Vec<StepFunction>,
    temperature: f64,
    alpha: f64,
    terminating_temperature: f64
}

impl SimulatedAnnealing {
    pub fn new(neighborhoods: Vec<Box<dyn NeighborhoodImpl>>, step_functions: Vec<StepFunction>) -> Self {
        SimulatedAnnealing {
            neighborhoods,
            step_functions,
            temperature: 1.0,
            alpha: 0.01,
            terminating_temperature: 0.01
        }
    }
}

impl Solver for SimulatedAnnealing {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        
    }

    fn to_string(&self) -> String {
        String::from("SimulatedAnnealing")
    }
}