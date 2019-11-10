use std::fmt;

#[derive(Debug)]
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

}