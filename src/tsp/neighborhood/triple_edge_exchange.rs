use std::rc::Rc;

use super::NeighborhoodImpl;
use crate::tsp::Solution;
use crate::tsp::TSPInstance;
use crate::rand::Rng;
use crate::modulo_pos;

pub struct TripleEdgeExchange {
    max_length: usize
}

impl TripleEdgeExchange {
    pub fn new(max_length: usize) -> Self {
        TripleEdgeExchange {
            max_length
        }
    }

    pub fn apply(solution: &mut Solution, start_idx: usize, first_block_length: usize, second_block_length: usize, delta_eval: bool) {
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

        solution.get_assignment_mut((start_idx) % number_of_vertices).set_driver(copy[0].driver());
        solution.get_assignment_mut((start_idx + second_block_length) % number_of_vertices).set_driver(copy[total_length].driver());
        solution.get_assignment_mut((start_idx + total_length) % number_of_vertices).set_driver(copy[first_block_length].driver());
        
        if !delta_eval {
            return;
        }

        let driver = copy[0].driver();
        let first_vertex = solution.get_assignment((start_idx - 1) % number_of_vertices).vertex();
        let old_destination = copy[0].vertex();
        let new_destination = solution.get_assignment(start_idx).vertex();
        let old_distance = solution.instance().get_vertex(first_vertex).get_weight(old_destination);
        let new_distance = solution.instance().get_vertex(first_vertex).get_weight(new_destination);
        solution.delta_evaluation(driver, old_distance - new_distance);

        
        let driver = copy[first_block_length].driver();
        let first_vertex = copy[first_block_length - 1].vertex();
        let old_destination = copy[first_block_length].vertex();
        let new_destination = solution.get_assignment((start_idx + total_length) % number_of_vertices).vertex();
        let old_distance = solution.instance().get_vertex(first_vertex).get_weight(old_destination);
        let new_distance = solution.instance().get_vertex(first_vertex).get_weight(new_destination);
        solution.delta_evaluation(driver, old_distance - new_distance);
        


        let driver = copy[total_length].driver();
        let first_vertex = copy[total_length - 1].vertex();
        println!("first_vertex: {}", first_vertex);
        let old_destination = copy[total_length].vertex();
        println!("old_dest: {}", old_destination);
        let new_destination = solution.get_assignment((start_idx + first_block_length) % number_of_vertices).vertex();
        println!("new_dest: {}", new_destination);
        let old_distance = solution.instance().get_vertex(first_vertex).get_weight(old_destination);
        let new_distance = solution.instance().get_vertex(first_vertex).get_weight(new_destination);
        solution.delta_evaluation(driver, old_distance - new_distance);
    }

    pub fn get_delta(solution: &Solution, start_idx: usize, first_block_length: usize, second_block_length: usize) -> isize {
        let number_of_vertices = solution.instance().number_of_vertices();
        let total_length = first_block_length + second_block_length;
        let ass_0 = solution.get_assignment((start_idx + total_length) % number_of_vertices);
        let ass_1 = solution.get_assignment(modulo_pos(start_idx as isize - 1, number_of_vertices));
        let ass_2 = solution.get_assignment(start_idx);
        let ass_3 = solution.get_assignment((start_idx + first_block_length - 1) % number_of_vertices);
        let ass_4 = solution.get_assignment((start_idx + first_block_length) % number_of_vertices);
        let ass_5 = solution.get_assignment((start_idx + total_length - 1) % number_of_vertices);
        
        println!("v1: {}, v2: {}", ass_1.vertex(), ass_2.vertex());
        let e_1 = solution.instance().get_vertex(ass_1.vertex()).get_weight(ass_2.vertex());
        let e_2 = solution.instance().get_vertex(ass_3.vertex()).get_weight(ass_4.vertex());
        let e_3 = solution.instance().get_vertex(ass_5.vertex()).get_weight(ass_0.vertex());
        let e_4 = solution.instance().get_vertex(ass_1.vertex()).get_weight(ass_4.vertex());
        let e_5 = solution.instance().get_vertex(ass_3.vertex()).get_weight(ass_0.vertex());
        let e_6 = solution.instance().get_vertex(ass_2.vertex()).get_weight(ass_5.vertex());
        println!("e_1: {}", e_1);
        println!("e_2: {}", e_2);
        println!("e_3: {}", e_3);
        println!("e_4: {}", e_4);
        println!("e_5: {}", e_5);
        println!("e_6: {}", e_6);

        let desired = solution.instance().desired_travel_distance();

        let driver_1 = solution.get_driver_distance(ass_2.driver());
        let driver_2 = solution.get_driver_distance(ass_4.driver());
        let driver_3 = solution.get_driver_distance(ass_0.driver());
        println!("driver_1: {}", driver_1);
        println!("driver_2: {}", driver_2);
        println!("driver_3: {}", driver_3);

        let mut driver_distances = solution.driver_distances().clone();
        driver_distances[ass_2.driver()] = driver_distances[ass_2.driver()] - e_1 + e_4;
        driver_distances[ass_4.driver()] = driver_distances[ass_4.driver()] - e_2 + e_5;
        driver_distances[ass_0.driver()] = driver_distances[ass_0.driver()] - e_3 + e_6;
        
        let mut delta = 0;
        for i in 0..driver_distances.len() {
            delta += (desired - driver_distances[i]).pow(2) - 
                (desired - solution.get_driver_distance(i)).pow(2);
        }
        delta
    }
}

impl NeighborhoodImpl for TripleEdgeExchange {
    fn get_random_neighbor(&self, solution: &mut Solution, delta_eval: bool) {
        let start = rand::thread_rng().gen_range(0, solution.instance().number_of_vertices());
        let first_length = rand::thread_rng().gen_range(1, self.max_length + 1);
        let second_length = rand::thread_rng().gen_range(1, self.max_length + 1);
        TripleEdgeExchange::apply(solution, start, first_length, second_length, delta_eval);
    }

    fn get_best_improving_neighbor(&self, solution: &mut Solution, delta_eval: bool) {

    }

    fn get_first_improving_neighbor(&self, solution: &mut Solution, delta_eval: bool) {
        
    }

    fn to_string(&self) -> String {
        format!("TripleEdgeExchange.{}", self.max_length)
    }
}

// #[test]
// fn test_double_edge_thingy() {
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
    let first_length = rand::thread_rng().gen_range(1, 4);
    let second_length = rand::thread_rng().gen_range(1, 4);
    let new_val = TripleEdgeExchange::get_delta(&solution, start, first_length, second_length) + solution.objective_value();
    TripleEdgeExchange::apply(&mut solution, start, first_length, second_length, true);
    assert_eq!(new_val, solution.objective_value());
}