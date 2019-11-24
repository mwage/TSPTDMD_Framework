use std::rc::Rc;

use super::NeighborhoodImpl;
use crate::tsp::Solution;
use crate::tsp::TSPInstance;
use crate::rand::Rng;
use crate::modulo_pos;

pub struct DoubleEdgeExchange {
    max_length: usize
}

impl DoubleEdgeExchange {
    pub fn new(max_length: usize) -> Self {
        DoubleEdgeExchange {
            max_length
        }
    }

    pub fn apply(solution: &mut Solution, start_idx: usize, length: usize, delta_eval: bool) {

        // TODO: Only set instance, calc distances on the fly
        let number_of_vertices = solution.instance().number_of_vertices();
        let start_idx = modulo_pos(start_idx as isize - 1, number_of_vertices);
        assert!(length != 0);
        let mut copy = Vec::with_capacity(length + 1);
        for i in start_idx..start_idx + length + 1 {
            copy.push(solution.get_assignment(i % number_of_vertices).clone());
        }
        for i in 0..copy.len() {
            solution.get_assignment_mut((start_idx + length - i) % number_of_vertices).set_vertex(copy[i].vertex());
            if i == copy.len() - 1 {
                continue;
            }
            solution.get_assignment_mut((start_idx + length - i) % number_of_vertices).set_driver(copy[i + 1].driver());
        }

        if !delta_eval {
            return;
        }

        let prev_vertex = solution.get_assignment(modulo_pos(start_idx as isize - 1, number_of_vertices)).vertex();
        let old_distance = solution.instance().get_vertex(prev_vertex).get_weight(copy[0].vertex());   // Old distance of d0 to start vertex
        let new_vertex = solution.get_assignment(start_idx).vertex();
        let new_distance = solution.instance().get_vertex(prev_vertex).get_weight(new_vertex);
        solution.delta_evaluation(solution.get_assignment(start_idx).driver(), old_distance - new_distance);

        let next_vertex = solution.get_assignment((start_idx + length + 1) % number_of_vertices).vertex();
        let old_distance = solution.instance().get_vertex(next_vertex).get_weight(copy[copy.len() - 1].vertex());   // Old distance of d0 to start vertex
        let new_vertex = solution.get_assignment((start_idx + length) % number_of_vertices).vertex();
        let new_distance = solution.instance().get_vertex(next_vertex).get_weight(new_vertex);
        solution.delta_evaluation(solution.get_assignment((start_idx + length + 1) % number_of_vertices).driver(), old_distance - new_distance);
    }

    pub fn get_delta(solution: &Solution, start_idx: usize, length: usize) -> isize {
        let number_of_vertices = solution.instance().number_of_vertices();
        let start_idx = modulo_pos(start_idx as isize - 1, number_of_vertices);
        let prev_ass = solution.get_assignment(modulo_pos(start_idx as isize - 1, number_of_vertices));
        let start_ass = solution.get_assignment(start_idx);
        let end_ass = solution.get_assignment((start_idx + length) % number_of_vertices);
        let next_ass = solution.get_assignment((start_idx + length + 1) % number_of_vertices);
        
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
        delta
    }
}

impl NeighborhoodImpl for DoubleEdgeExchange {
    fn get_random_neighbor(&self, solution: &mut Solution, delta_eval: bool) -> bool {
        let start = rand::thread_rng().gen_range(0, solution.instance().number_of_vertices());
        let length = rand::thread_rng().gen_range(1, self.max_length + 1);
        DoubleEdgeExchange::apply(solution, start, length, delta_eval);
        true
    }

    fn get_best_improving_neighbor(&self, solution: &mut Solution, delta_eval: bool) -> bool {
        let number_of_vertices = solution.instance().number_of_vertices();
        let mut best_solution: (usize, usize, isize) = (0, 0, 0);
        for start_idx in 0..number_of_vertices {
            for block_length in 1..self.max_length {
                let delta = DoubleEdgeExchange::get_delta(solution, start_idx, block_length);
                if delta < best_solution.2 {
                    best_solution = (start_idx, block_length, delta);
                }
            }
        }

        if best_solution.2 > 0 {
            DoubleEdgeExchange::apply(
                solution, best_solution.0, best_solution.1, delta_eval);
            return true;
        }

        false
    }
    
    fn get_first_improving_neighbor(&self, solution: &mut Solution, delta_eval: bool) -> bool {
        let number_of_vertices = solution.instance().number_of_vertices();
        for start_idx in 0..number_of_vertices {
            for block_length in 1..self.max_length {
                let delta = DoubleEdgeExchange::get_delta(solution, start_idx, block_length);
                if delta < 0 {
                    DoubleEdgeExchange::apply(
                        solution, start_idx, block_length, delta_eval);
                    return true;
                }
            }
        }
        false
    }

    fn to_string(&self) -> String {
        format!("DoubleEdgeExchange.{}", self.max_length)
    }
}
    

// #[test]
// fn test_double_edge_thingy() {
//     let neighborhood = DoubleEdgeExchange::new(0);
//     let vertices = 5;
//     let instance = TSPInstance::new(vertices, vertices, 10);
//     let mut solution = Solution::new(Rc::new(instance));
//     for i in 0..vertices as u32 {
//         solution.add_assignment(i, i, 10);
//         assert_eq!(solution.get_assignment(i as usize).driver(), i);
//         assert_eq!(solution.get_assignment(i as usize).vertex(), i);
//     }
//     assert_eq!(solution.assignments().len(), vertices);
//     DoubleEdgeExchange::apply(&mut solution, 1, 2, false);
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
//     DoubleEdgeExchange::apply(&mut solution, 3, 2, false);
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

#[test]
fn test_delta_eval() {
    let vertices = 5;
    let mut instance = TSPInstance::new(vertices, vertices, 100);
    instance.add_edge(0, 1, 10);
    instance.add_edge(0, 2, 5);
    instance.add_edge(0, 3, 106);
    instance.add_edge(0, 4, 52);
    instance.add_edge(1, 2, 24);
    instance.add_edge(1, 3, 17);
    instance.add_edge(1, 4, 20);
    instance.add_edge(2, 3, 17);
    instance.add_edge(2, 4, 20);
    instance.add_edge(3, 4, 47);

    let mut solution = Solution::new(Rc::new(instance));
    for i in 0..vertices {
        solution.add_assignment(i, i, 10);
        assert_eq!(solution.get_assignment(i).driver(), i);
        assert_eq!(solution.get_assignment(i).vertex(), i);
    }
    solution.calculate_objective_value();
    DoubleEdgeExchange::apply(&mut solution, 1, 2, true);
    let x = solution.objective_value();
    solution.calculate_objective_value();
    assert_eq!(x, solution.objective_value());
}


#[test]
fn test_delta() {
    let instance = TSPInstance::new_random(10, 3, 100, 50);
    let mut solution = Solution::new_random(Rc::new(instance));
    solution.calculate_objective_value();
    let start = rand::thread_rng().gen_range(0, solution.instance().number_of_vertices());
    let length = rand::thread_rng().gen_range(1, 4);
    println!("start: {}", start);
    println!("length: {}", length);

    let new_val = DoubleEdgeExchange::get_delta(&solution, start, length) + solution.objective_value();
    DoubleEdgeExchange::apply(&mut solution, start, length, true);
    assert_eq!(new_val, solution.objective_value());
}