use std::fmt;

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