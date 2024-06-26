use std::rc::Rc;
use std::cmp;

use super::NeighborhoodImpl;
use crate::tsp::Solution;
use crate::tsp::TSPInstance;
use crate::tsp::io::Logger;
use crate::rand::Rng;
use crate::modulo_pos;

pub struct TripleEdgeExchange {
    max_length: Option<usize>,
    stored_move: Option<TEMove>
}

impl TripleEdgeExchange {
    pub fn new(max_length: Option<usize>) -> Self {
        TripleEdgeExchange {
            max_length,
            stored_move: None
        }
    }

    fn stored_move(&self) -> &TEMove {
        match &self.stored_move {
            Some(x) => &x,
            None => panic!("Attempted to set non-initialized neighbor.")
        }        
    }
    
    fn apply(&mut self, solution: &mut Solution) {
        let (start_idx, first_block_length, second_block_length, delta, distances) = self.stored_move().to_tuple();
        let number_of_vertices = solution.instance().number_of_vertices();
        let total_length = first_block_length + second_block_length;

        assert!(total_length < number_of_vertices);
        assert_ne!(first_block_length, 0);
        assert_ne!(second_block_length, 0);

        let mut copy = Vec::with_capacity(total_length);
        for i in start_idx..start_idx + total_length + 1 {
            copy.push(solution.get_assignment(i % number_of_vertices).clone());
        }
        for i in 0..second_block_length {
            solution.get_assignment_mut((start_idx + i) % number_of_vertices).from_assignment(&copy[first_block_length + i]);
        }
        for i in 0..first_block_length {
            solution.get_assignment_mut((start_idx + second_block_length + i) % number_of_vertices).from_assignment(&copy[i]);
        }

        solution.get_assignment_mut(start_idx % number_of_vertices).set_driver(copy[0].driver());
        solution.get_assignment_mut((start_idx + second_block_length) % number_of_vertices).set_driver(copy[total_length].driver());
        solution.get_assignment_mut((start_idx + total_length) % number_of_vertices).set_driver(copy[first_block_length].driver());
        
        solution.delta_evaluation(delta, distances);
    }

    fn evaluate_move(&self, solution: &Solution, start_idx: usize, first_block_length: usize, second_block_length: usize) -> TEMove {
        let number_of_vertices = solution.instance().number_of_vertices();
        let first_block_length = first_block_length + 1;    // Transform number of edges to number of nodes
        let second_block_length = second_block_length + 1;  // for easier indexing
        let total_length = first_block_length + second_block_length;

        let ass_0 = solution.get_assignment((start_idx + total_length) % number_of_vertices);
        let ass_1 = solution.get_assignment(modulo_pos(start_idx as isize - 1, number_of_vertices));
        let ass_2 = solution.get_assignment(start_idx);
        let ass_3 = solution.get_assignment((start_idx + first_block_length - 1) % number_of_vertices);
        let ass_4 = solution.get_assignment((start_idx + first_block_length) % number_of_vertices);
        let ass_5 = solution.get_assignment((start_idx + total_length - 1) % number_of_vertices);

        let e_1 = solution.instance().get_vertex(ass_1.vertex()).get_weight(ass_2.vertex());
        let e_2 = solution.instance().get_vertex(ass_3.vertex()).get_weight(ass_4.vertex());
        let e_3 = solution.instance().get_vertex(ass_5.vertex()).get_weight(ass_0.vertex());
        let e_4 = solution.instance().get_vertex(ass_1.vertex()).get_weight(ass_4.vertex());
        let e_5 = solution.instance().get_vertex(ass_3.vertex()).get_weight(ass_0.vertex());
        let e_6 = solution.instance().get_vertex(ass_2.vertex()).get_weight(ass_5.vertex());



        let desired = solution.instance().desired_travel_distance();

        let mut updated_driver_distances = solution.driver_distances().clone();
        updated_driver_distances[ass_2.driver()] = updated_driver_distances[ass_2.driver()] - e_1 + e_4;
        updated_driver_distances[ass_4.driver()] = updated_driver_distances[ass_4.driver()] - e_2 + e_5;
        updated_driver_distances[ass_0.driver()] = updated_driver_distances[ass_0.driver()] - e_3 + e_6;
        
        let mut delta = 0;
        for i in 0..updated_driver_distances.len() {
            delta += i128::from(desired - updated_driver_distances[i]).pow(2) - 
                    i128::from(desired - solution.get_driver_distance(i)).pow(2);
        }

        TEMove::new(start_idx, first_block_length, second_block_length, delta, updated_driver_distances)
    }

    fn calculate_max_length(&self, instance: &TSPInstance) -> usize {
        let max = (instance.number_of_vertices() - 1) / 2;
        if let Some(length) = self.max_length {
            cmp::min(length, max)
        } else {
            max
        }
    }
}

impl NeighborhoodImpl for TripleEdgeExchange {
    fn get_random_neighbor(&mut self, solution: &Solution) -> bool {
        let max_length = self.calculate_max_length(solution.instance());
        let start = rand::thread_rng().gen_range(0, solution.instance().number_of_vertices());
        let first_length = rand::thread_rng().gen_range(1, max_length);
        let second_length = rand::thread_rng().gen_range(0, max_length);
        self.stored_move = Some(self.evaluate_move(solution, start, first_length, second_length));

        true
    }

    fn get_best_improving_neighbor(&mut self, solution: &Solution, logger: &Logger) -> bool {
        self.stored_move = None;
        let max_length = self.calculate_max_length(solution.instance());
        let number_of_vertices = solution.instance().number_of_vertices();
        for start_idx in 0..number_of_vertices {
            for first_block_length in 1..max_length {
                for second_block_length in 0..max_length {
                    let te_move = self.evaluate_move(solution, start_idx, first_block_length, second_block_length);
                    if let Some(delta) = self.delta() {  
                        if te_move.delta() < delta {
                            self.stored_move = Some(te_move);
                        }
                    } else {
                        self.stored_move = Some(te_move);
                    }

                    if logger.get_elapsed() >= crate::TIME_LIMIT {
                        return match &self.stored_move {
                            Some(te_move) => te_move.delta() < 0,
                            None => false
                        };
                    }
                }
            }
        }

        match &self.stored_move {
            Some(te_move) => te_move.delta() < 0,
            None => false
        }
    }

    fn get_first_improving_neighbor(&mut self, solution: &Solution, logger: &Logger) -> bool {
        self.stored_move = None;
        let max_length = self.calculate_max_length(solution.instance());
        let number_of_vertices = solution.instance().number_of_vertices();
        for start_idx in 0..number_of_vertices {
            for first_block_length in 1..max_length {
                for second_block_length in 1..max_length {
                    let te_move = self.evaluate_move(solution, start_idx, first_block_length, second_block_length);

                    if te_move.delta() < 0 {
                        self.stored_move = Some(te_move);
                        return true;
                    }
                    if logger.get_elapsed() >= crate::TIME_LIMIT {
                        return false;
                    }
                }
            }
        }

        false
    }

    fn set_neighbor(&mut self, solution: &mut Solution) {
        self.apply(solution);
        self.stored_move = None;
    }

    fn delta(&self) -> Option<i128> {
        match &self.stored_move {
            Some(x) => Some(x.delta),
            None => None
        }
    }

    fn to_string(&self) -> String {
        match self.max_length {
            Some(x) => format!("te-{}", x),
            _ => String::from("te-max")
        }
    }
}

pub struct TEMove {
    start_idx: usize,
    first_block_length: usize,
    second_block_length: usize,
    delta: i128,
    distances: Vec<i64>
}

impl TEMove {
    pub fn new(start_idx: usize, first_block_length: usize, second_block_length: usize, delta: i128, distances: Vec<i64>) -> Self {
        TEMove {
            start_idx,
            first_block_length,
            second_block_length,
            delta,
            distances
        }
    }

    pub fn delta(&self) -> i128 {
        self.delta
    }

    pub fn to_tuple(&self) -> (usize, usize, usize, i128, Vec<i64>) {
        (self.start_idx, self.first_block_length, self.second_block_length, self.delta, self.distances.clone())
    }
}

#[test]
fn test_delta() {
    for _ in 0..100 {
        let instance = TSPInstance::new_random(10, 10, 100, 50);
        let mut solution = Solution::new_random(Rc::new(instance));
        solution.calculate_objective_value();
        let start = rand::thread_rng().gen_range(0, solution.instance().number_of_vertices());
        let first_length = rand::thread_rng().gen_range(1, 3);
        let second_length = rand::thread_rng().gen_range(1, 3);
    
        let mut triple_edge_exchange = TripleEdgeExchange::new(Some(3));
        triple_edge_exchange.stored_move = Some(triple_edge_exchange.evaluate_move(&solution, start, first_length, second_length));
        triple_edge_exchange.apply(&mut solution);
        let new_val = solution.objective_value();
        solution.calculate_objective_value_from_scratch();
        assert_eq!(new_val, solution.objective_value());
    }
}