use std::rc::Rc;

use super::NeighborhoodImpl;
use crate::tsp::Solution;
use crate::tsp::TSPInstance;
use crate::rand::Rng;
use crate::modulo_pos;

pub struct DoubleEdgeExchange {
    max_length: usize,
    stored_move: Option<DEMove>
}

impl DoubleEdgeExchange {
    pub fn new(max_length: usize) -> Self {
        DoubleEdgeExchange {
            max_length,
            stored_move: None
        }
    }

    pub fn delta(&self) -> Option<isize> {
        match &self.stored_move {
            Some(x) => Some(x.delta),
            None => None
        }
    }

    fn stored_move(&self) -> &DEMove {
        match &self.stored_move {
            Some(x) => &x,
            None => panic!("Attempted to set non-initialized neighbor.")
        }        
    }

    fn apply(&mut self, solution: &mut Solution, delta_eval: bool) {
        let (start_idx, block_length, delta, distances) = self.stored_move().to_tuple();
        let number_of_vertices = solution.instance().number_of_vertices();

        let mut copy = Vec::with_capacity(block_length + 1);
        for i in start_idx..start_idx + block_length + 1 {
            copy.push(solution.get_assignment(i % number_of_vertices).clone());
        }
        for i in 0..copy.len() {
            solution.get_assignment_mut((start_idx + block_length - i) % number_of_vertices).set_vertex(copy[i].vertex());
            if i == copy.len() - 1 {
                continue;
            }
            solution.get_assignment_mut((start_idx + block_length - i) % number_of_vertices).set_driver(copy[i + 1].driver());
        }

        if delta_eval {
            solution.delta_evaluation(delta, distances);
        }
    }

    fn evaluate_move(&self, solution: &Solution, start_idx: usize, block_length: usize) -> DEMove {
        assert!(block_length != 0);
        let number_of_vertices = solution.instance().number_of_vertices();
        let prev_ass = solution.get_assignment(modulo_pos(start_idx as isize - 1, number_of_vertices));
        let start_ass = solution.get_assignment(start_idx);
        let end_ass = solution.get_assignment((start_idx + block_length) % number_of_vertices);
        let next_ass = solution.get_assignment((start_idx + block_length + 1) % number_of_vertices);
        
        let e_1 = solution.instance().get_vertex(prev_ass.vertex()).get_weight(start_ass.vertex());
        let e_2 = solution.instance().get_vertex(next_ass.vertex()).get_weight(end_ass.vertex());
        let e_3 = solution.instance().get_vertex(prev_ass.vertex()).get_weight(end_ass.vertex());
        let e_4 = solution.instance().get_vertex(start_ass.vertex()).get_weight(next_ass.vertex());
        let desired = solution.instance().desired_travel_distance();

        let mut driver_distances = solution.driver_distances().clone();
        driver_distances[start_ass.driver()] = driver_distances[start_ass.driver()] - e_1 + e_3;
        driver_distances[next_ass.driver()] = driver_distances[next_ass.driver()] - e_2 + e_4;
        
        let mut delta = 0;
        for i in 0..driver_distances.len() {
            delta += (desired - driver_distances[i]).pow(2) - 
                (desired - solution.get_driver_distance(i)).pow(2);
        }
        DEMove::new(start_idx, block_length, delta, driver_distances)
    }
}

impl NeighborhoodImpl for DoubleEdgeExchange {
    fn get_random_neighbor(&mut self, solution: &Solution, delta_eval: bool) -> bool {
        let start_idx = rand::thread_rng().gen_range(0, solution.instance().number_of_vertices());
        let block_length = rand::thread_rng().gen_range(1, self.max_length + 1);
        self.stored_move = Some(self.evaluate_move(solution, start_idx, block_length));
        true
    }

    fn get_best_improving_neighbor(&mut self, solution: &Solution, delta_eval: bool) -> bool {
        let number_of_vertices = solution.instance().number_of_vertices();
        for start_idx in 0..number_of_vertices {
            for block_length in 1..self.max_length {
                let de_move = self.evaluate_move(solution, start_idx, block_length);

                // If move is not set or delta < delta of stored solution => update stored move
                if let Some(delta) = self.delta() {  
                    if de_move.delta() >= delta {
                        continue;
                    }
                }

                self.stored_move = Some(de_move);
            }
        }

        match self.stored_move {
            Some(_) => true,
            None => false
        }
    }
    
    fn get_first_improving_neighbor(&mut self, solution: &Solution, delta_eval: bool) -> bool {
        let number_of_vertices = solution.instance().number_of_vertices();
        for start_idx in 0..number_of_vertices {
            for block_length in 1..self.max_length {
                let de_move = self.evaluate_move(solution, start_idx, block_length);
                if de_move.delta() < 0 {
                    self.stored_move = Some(de_move);
                    return true;
                }
            }
        }
        false
    }

    fn set_neighbor(&mut self, solution: &mut Solution, delta_eval: bool) {
        self.apply(solution, delta_eval);
        self.stored_move = None;
    }

    fn to_string(&self) -> String {
        format!("DoubleEdgeExchange.{}", self.max_length)
    }
}

struct DEMove {
    start_idx: usize,
    block_length: usize,
    delta: isize,
    distances: Vec<isize>
}

impl DEMove {
    pub fn new(start_idx: usize, block_length: usize, delta: isize, distances: Vec<isize>) -> Self {
        DEMove {
            start_idx,
            block_length,
            delta,
            distances
        }
    }

    pub fn delta(&self) -> isize {
        self.delta
    }

    pub fn to_tuple(&self) -> (usize, usize, isize, Vec<isize>) {
        (self.start_idx, self.block_length, self.delta, self.distances.clone())
    }
}


#[test]
fn test_delta() {
    let instance = TSPInstance::new_random(10, 3, 100, 50);
    let mut solution = Solution::new_random(Rc::new(instance));
    solution.calculate_objective_value();
    let start = rand::thread_rng().gen_range(0, solution.instance().number_of_vertices());
    let length = rand::thread_rng().gen_range(1, 4);

    let mut double_edge_exchange = DoubleEdgeExchange::new(4);
    double_edge_exchange.stored_move = Some(double_edge_exchange.evaluate_move(&solution, start, length));
    println!("{:?}", solution.driver_distances());
    println!("{}", solution.objective_value());
    double_edge_exchange.apply(&mut solution, true);
    println!("{:?}", solution.driver_distances());
    println!("{}", solution.objective_value());
    let x = solution.objective_value();
    solution.calculate_objective_value_from_scratch();
    println!("{:?}", solution.driver_distances());
    println!("{}", solution.objective_value());
    assert_eq!(x, solution.objective_value());
}