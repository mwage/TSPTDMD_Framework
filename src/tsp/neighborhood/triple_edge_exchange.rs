use std::rc::Rc;

use super::NeighborhoodImpl;
use crate::tsp::Solution;
use crate::tsp::TSPInstance;
use crate::rand::Rng;
use crate::modulo_pos;

pub struct TripleEdgeExchange {
    max_length: usize,
    stored_move: Option<TEMove>
}

impl TripleEdgeExchange {
    pub fn new(max_length: usize) -> Self {
        TripleEdgeExchange {
            max_length,
            stored_move: None
        }
    }

    pub fn stored_move(&self) -> &TEMove {
        match &self.stored_move {
            Some(x) => &x,
            None => panic!("Attempted to set non-initialized neighbor.")
        }        
    }

    pub fn set_move(&mut self, new_move: TEMove) {
        self.stored_move = Some(new_move);
    }

    pub fn apply(&mut self, solution: &mut Solution, delta_eval: bool) {
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
        
        if delta_eval {
            solution.delta_evaluation(delta, distances);
        }
    }

    pub fn evaluate_move(&self, solution: &Solution, start_idx: usize, first_block_length: usize, second_block_length: usize) -> TEMove {
        // println!("Start of delta!");
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
            delta += (desired - updated_driver_distances[i]).pow(2) - 
                (desired - solution.get_driver_distance(i)).pow(2);
        }

        TEMove::new(start_idx, first_block_length, second_block_length, delta, updated_driver_distances)
    }
}

impl NeighborhoodImpl for TripleEdgeExchange {
    fn get_random_neighbor(&mut self, solution: &Solution, delta_eval: bool) -> bool {
        let start = rand::thread_rng().gen_range(0, solution.instance().number_of_vertices());
        let first_length = rand::thread_rng().gen_range(1, self.max_length + 1);
        // TODO: get max_length from instance size?
        let second_length = rand::thread_rng().gen_range(1, self.max_length + 1);
        self.stored_move = Some(self.evaluate_move(solution, start, first_length, second_length));

        true
    }

    fn get_best_improving_neighbor(&mut self, solution: &Solution, delta_eval: bool) -> bool {
        let number_of_vertices = solution.instance().number_of_vertices();
        let mut best_solution: (usize, usize, usize, isize) = (0, 0, 0, 0);
        for start_idx in 0..number_of_vertices {
            for first_block_length in 1..self.max_length {
                for second_block_length in 1..self.max_length {
                    let te_move = self.evaluate_move(solution, start_idx, first_block_length, second_block_length);
                    if let Some(delta) = self.delta() {  
                        if te_move.delta() >= delta {
                            continue;
                        }
                    }

                    self.stored_move = Some(te_move);
                }
            }
        }

        match &self.stored_move {
            Some(te_move) => te_move.delta() < 0,
            None => false
        }
    }

    fn get_first_improving_neighbor(&mut self, solution: &Solution, delta_eval: bool) -> bool {
        let number_of_vertices = solution.instance().number_of_vertices();
        for start_idx in 0..number_of_vertices {
            for first_block_length in 1..self.max_length {
                for second_block_length in 1..self.max_length {
                    let te_move = self.evaluate_move(solution, start_idx, first_block_length, second_block_length);

                    if te_move.delta() < 0 {
                        self.stored_move = Some(te_move);
                        return true;
                    }
                }
            }
        }

        false
    }

    fn set_neighbor(&mut self, solution: &mut Solution, delta_eval: bool) {
        self.apply(solution, delta_eval);
        self.stored_move = None;
    }

    fn delta(&self) -> Option<isize> {
        match &self.stored_move {
            Some(x) => Some(x.delta),
            None => None
        }
    }

    fn to_string(&self) -> String {
        format!("TripleEdgeExchange.{}", self.max_length)
    }
}

pub struct TEMove {
    start_idx: usize,
    first_block_length: usize,
    second_block_length: usize,
    delta: isize,
    distances: Vec<isize>
}

impl TEMove {
    pub fn new(start_idx: usize, first_block_length: usize, second_block_length: usize, delta: isize, distances: Vec<isize>) -> Self {
        TEMove {
            start_idx,
            first_block_length,
            second_block_length,
            delta,
            distances
        }
    }

    pub fn delta(&self) -> isize {
        self.delta
    }

    pub fn to_tuple(&self) -> (usize, usize, usize, isize, Vec<isize>) {
        (self.start_idx, self.first_block_length, self.second_block_length, self.delta, self.distances.clone())
    }
}

// #[test]
// fn test_triple_edge_thingy() {
//     let neighborhood = TripleEdgeExchange::new(0);
//     let vertices = 5;
//     let instance = TSPInstance::new(vertices, vertices, 10);
//     let mut solution = Solution::new(Rc::new(instance));
//     for i in 0..vertices as u32 {
//         solution.add_assignment(i, i, 10);
//         assert_eq!(solution.get_assignment(i as usize).driver(), i);
//         assert_eq!(solution.get_assignment(i as usize).vertex(), i);
//     }
//     assert_eq!(solution.assignments().len(), vertices);
//     TripleEdgeExchange::apply(&mut solution, 1, 2, false);
//     assert_eq!(solution.get_assignment(0).vertex(), 0);
//     assert_eq!(solution.get_assignment(0).driver(), 0);
//     assert_eq!(solution.get_assignment(1).vertex(), 3);
//     assert_eq!(solution.get_assignment(1).driver(), 1);
//     assert_eq!(solution.get_assignment(2).vertex(), 2);
//     assert_eq!(solution.get_assignment(2).driver(), 3);
//     assert_eq!(solution.get_assignment(3).vertex(), 1);
//     assert_eq!(solution.get_assignment(3).driver(), 2);
//     assert_eq!(solution.get_assignment(4).vertex(), 4);
//     assert_eq!(solution.get_assignment(4).driver(), 4);

//     // Test overflow
//     let instance = TSPInstance::new(vertices, vertices, 10);
//     let mut solution = Solution::new(Rc::new(instance));
//     for i in 0..vertices as u32 {
//         solution.add_assignment(i, i, 10);
//         assert_eq!(solution.get_assignment(i as usize).driver(), i);
//         assert_eq!(solution.get_assignment(i as usize).vertex(), i);
//     }
//     assert_eq!(solution.assignments().len(), vertices);
//     TripleEdgeExchange::apply(&mut solution, 3, 2, false);
//     assert_eq!(solution.get_assignment(0).vertex(), 3);
//     assert_eq!(solution.get_assignment(0).driver(), 4);
//     assert_eq!(solution.get_assignment(1).vertex(), 1);
//     assert_eq!(solution.get_assignment(1).driver(), 1);
//     assert_eq!(solution.get_assignment(2).vertex(), 2);
//     assert_eq!(solution.get_assignment(2).driver(), 2);
//     assert_eq!(solution.get_assignment(3).vertex(), 0);
//     assert_eq!(solution.get_assignment(3).driver(), 3);
//     assert_eq!(solution.get_assignment(4).vertex(), 4);
//     assert_eq!(solution.get_assignment(4).driver(), 0);
// }

// #[test]
// fn test_delta_eval() {
//     let vertices = 5;
//     let mut instance = TSPInstance::new(vertices, vertices, 100);
//     instance.add_edge(0, 1, 10);
//     instance.add_edge(0, 2, 5);
//     instance.add_edge(0, 3, 106);
//     instance.add_edge(0, 4, 52);
//     instance.add_edge(1, 2, 24);
//     instance.add_edge(1, 3, 17);
//     instance.add_edge(1, 4, 20);
//     instance.add_edge(2, 3, 17);
//     instance.add_edge(2, 4, 20);
//     instance.add_edge(3, 4, 47);

//     let mut solution = Solution::new(Rc::new(instance));
//     for i in 0..vertices as u32 {
//         solution.add_assignment(i, i, 10);
//         assert_eq!(solution.get_assignment(i as usize).driver(), i);
//         assert_eq!(solution.get_assignment(i as usize).vertex(), i);
//     }
//     solution.calculate_objective_value();
//     DoubleEdgeExchange::apply(&mut solution, 1, 2, true);
//     let x = solution.objective_value();
//     solution.calculate_objective_value();
//     assert_eq!(x, solution.objective_value());
// }

#[test]
fn test_delta() {
    let instance = TSPInstance::new_random(10, 10, 100, 50);
    let mut solution = Solution::new_random(Rc::new(instance));
    solution.calculate_objective_value();
    let start = rand::thread_rng().gen_range(0, solution.instance().number_of_vertices());
    let first_length = rand::thread_rng().gen_range(1, 3);
    let second_length = rand::thread_rng().gen_range(1, 3);

    let mut triple_edge_exchange = TripleEdgeExchange::new(3);
    triple_edge_exchange.stored_move = Some(triple_edge_exchange.evaluate_move(&solution, start, first_length, second_length));
    let new_val = triple_edge_exchange.delta().unwrap() + solution.objective_value();
    triple_edge_exchange.apply(&mut solution, true);

    assert_eq!(new_val, solution.objective_value());
}