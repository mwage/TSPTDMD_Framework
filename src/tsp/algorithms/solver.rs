use std::fmt;

use crate::tsp::TSPInstance;
use crate::tsp::Logger;

#[derive(Debug, Clone)]
pub enum Algorithm {
    Greedy,
    RandomGreedy
}

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Solver {
    fn solve(&mut self, instance: &mut TSPInstance, logger: &Logger);
}

#[derive(Debug, Clone)]
pub enum NeighborhoodFunction {
    Flip
}

impl fmt::Display for NeighborhoodFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Neighborhood {

}

#[derive(Debug, Clone)]
pub enum StepFunction {
    BestImprovement
}

impl fmt::Display for StepFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait StepFunctionImpl {
    
}