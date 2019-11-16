mod flip;

pub use flip::Flip;


use std::fmt;

#[derive(Debug, Clone)]
pub enum Neighborhood {
    Flip
}

impl fmt::Display for Neighborhood {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
    
pub trait NeighborhoodImpl {

}
