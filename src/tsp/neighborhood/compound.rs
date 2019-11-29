use super::DoubleEdgeExchange;
use super::TripleEdgeExchange;
use super::DriverFlip;
use super::Neighborhood;
use super::NeighborhoodImpl;
use crate::tsp::Solution;
use crate::tsp::TSPInstance;
use crate::rand::Rng;

pub struct Compound {
    max_length: Option<usize>,
    double_edge_exchange: Box<dyn NeighborhoodImpl>,
    driver_flip: Box<dyn NeighborhoodImpl>,
    triple_edge_exchange: Box<dyn NeighborhoodImpl>,
    stored_move: Option<Neighborhood>
}

impl Compound {
    pub fn new(max_length: Option<usize>) -> Self {
        Compound {
            max_length,
            double_edge_exchange: Box::new(DoubleEdgeExchange::new(max_length)),
            driver_flip: Box::new(DriverFlip::new()),
            triple_edge_exchange: Box::new(TripleEdgeExchange::new(max_length)),
            stored_move: None
        }
    }
    
    fn stored_move(&self) -> &Box<dyn NeighborhoodImpl> {        
        match &self.stored_move {
            Some(x) => {
                match x {
                    Neighborhood::DoubleEdgeExchange(_) => &self.double_edge_exchange,
                    Neighborhood::TripleEdgeExchange(_) => &self.triple_edge_exchange,
                    Neighborhood::DriverFlip => &self.driver_flip,
                    _ => panic!("Invalid neighborhood selected.")
                }
            },
            None => panic!("Attempted to set non-initialized neighbor.")
        }
    }
    
    fn stored_move_mut(&mut self) -> &mut Box<dyn NeighborhoodImpl> {        
        match &mut self.stored_move {
            Some(x) => {
                match x {
                    Neighborhood::DoubleEdgeExchange(_) => &mut self.double_edge_exchange,
                    Neighborhood::TripleEdgeExchange(_) => &mut self.triple_edge_exchange,
                    Neighborhood::DriverFlip => &mut self.driver_flip,
                    _ => panic!("Invalid neighborhood selected.")
                }
            },
            None => panic!("Attempted to set non-initialized neighbor.")
        }
    }
    
    fn apply(&mut self, solution: &mut Solution, delta_eval: bool) {
        // TODO: get delta from neighborhood with last stored move
    }

    fn calculate_max_length(&self, instance: &TSPInstance) -> usize {
        if let Some(length) = self.max_length {
            length
        } else {
            instance.number_of_vertices() - 1
        }
    }
}

impl NeighborhoodImpl for Compound {
    fn get_random_neighbor(&mut self, solution: &Solution, delta_eval: bool) -> bool {
        self.triple_edge_exchange.get_random_neighbor(solution, delta_eval)
        // TODO: randomly select one
    }

    fn get_best_improving_neighbor(&mut self, solution: &Solution, delta_eval: bool) -> bool {
        self.triple_edge_exchange.get_random_neighbor(solution, delta_eval)
        // TODO: randomly select one
    }
    
    fn get_first_improving_neighbor(&mut self, solution: &Solution, delta_eval: bool) -> bool {
        self.triple_edge_exchange.get_random_neighbor(solution, delta_eval)
        // TODO: randomly select one
    }

    fn set_neighbor(&mut self, solution: &mut Solution, delta_eval: bool) {
        self.stored_move_mut().set_neighbor(solution, delta_eval);
        self.stored_move = None;
    }
    
    fn delta(&self) -> Option<isize> {
        self.stored_move().delta()
    }

    fn to_string(&self) -> String {
        match self.max_length {
            Some(x) => format!("Compound-{}", x),
            _ => String::from("Compound-Max")
        }        
    }
}