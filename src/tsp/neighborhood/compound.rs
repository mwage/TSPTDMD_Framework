use super::DoubleEdgeExchange;
use super::TripleEdgeExchange;
use super::DriverFlip;
use super::Neighborhood;
use super::NeighborhoodImpl;
use crate::tsp::Solution;
use crate::rand::Rng;
use crate::tsp::io::Logger;

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
        match &self.stored_move {
            Some(x) => match x {
                Neighborhood::DoubleEdgeExchange(_) => &mut self.double_edge_exchange,
                Neighborhood::TripleEdgeExchange(_) => &mut self.triple_edge_exchange,
                Neighborhood::DriverFlip => &mut self.driver_flip,
                _ => panic!("Invalid neighborhood selected.")
            }
            None => panic!("Attempted to set non-initialized neighbor.")
        }
    }

    fn select_neighborhood(&self) -> Neighborhood {
        match rand::thread_rng().gen_range(0, 3) {
            0 => Neighborhood::TripleEdgeExchange(None),
            1 => Neighborhood::DoubleEdgeExchange(None),
            2 => Neighborhood::DriverFlip,
            _ => panic!("Invalid number generated!")
        }
    }

    fn get_neighborhood_impl_mut(&mut self, neighborhood: &Neighborhood) -> &mut Box<dyn NeighborhoodImpl> {
        match neighborhood {
            Neighborhood::DoubleEdgeExchange(_) => &mut self.double_edge_exchange,
            Neighborhood::TripleEdgeExchange(_) => &mut self.triple_edge_exchange,
            Neighborhood::DriverFlip => &mut self.driver_flip,
            _ => panic!("Invalid neighborhood selected.")
        }
    }
}

impl NeighborhoodImpl for Compound {
    fn get_random_neighbor(&mut self, solution: &Solution, delta_eval: bool) -> bool {
        let neighborhood = self.select_neighborhood();
        let res = self.get_neighborhood_impl_mut(&neighborhood).get_random_neighbor(solution, delta_eval);
        self.stored_move = Some(neighborhood);
        res
    }

    fn get_best_improving_neighbor(&mut self, solution: &Solution, delta_eval: bool, logger: &Logger) -> bool {
        let neighborhood = self.select_neighborhood();
        let res = self.get_neighborhood_impl_mut(&neighborhood).get_best_improving_neighbor(solution, delta_eval, logger);
        self.stored_move = Some(neighborhood);
        res
    }
    
    fn get_first_improving_neighbor(&mut self, solution: &Solution, delta_eval: bool, logger: &Logger) -> bool {
        let neighborhood = self.select_neighborhood();
        let res = self.get_neighborhood_impl_mut(&neighborhood).get_first_improving_neighbor(solution, delta_eval, logger);
        self.stored_move = Some(neighborhood);
        res
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
            Some(x) => format!("comp-{}", x),
            _ => String::from("comp-max")
        }        
    }
}