use std::rc::Rc;

use super::NeighborhoodImpl;
use crate::tsp::Solution;
use crate::tsp::TSPInstance;

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
        let old_distance = solution.instance().get_vertex(first_vertex).get_weight(old_destination) as isize;
        let new_distance = solution.instance().get_vertex(first_vertex).get_weight(new_destination) as isize;
        solution.delta_evaluation(driver, old_distance - new_distance);

        
        let driver = copy[first_block_length].driver();
        let first_vertex = copy[first_block_length - 1].vertex();
        let old_destination = copy[first_block_length].vertex();
        let new_destination = solution.get_assignment((start_idx + total_length) % number_of_vertices).vertex();
        let old_distance = solution.instance().get_vertex(first_vertex).get_weight(old_destination) as isize;
        let new_distance = solution.instance().get_vertex(first_vertex).get_weight(new_destination) as isize;
        solution.delta_evaluation(driver, old_distance - new_distance);
                
        let driver = copy[total_length].driver();
        let first_vertex = copy[total_length - 1].vertex();
        let old_destination = copy[total_length].vertex();
        let new_destination = solution.get_assignment((start_idx + first_block_length) % number_of_vertices).vertex();
        let old_distance = solution.instance().get_vertex(first_vertex).get_weight(old_destination) as isize;
        let new_distance = solution.instance().get_vertex(first_vertex).get_weight(new_destination) as isize;
        solution.delta_evaluation(driver, old_distance - new_distance);
    }
}

impl NeighborhoodImpl for TripleEdgeExchange {
    fn get_random_neighbor(&self, solution: &mut Solution) {

    }
    fn get_best_improving_neighbor(&self, solution: &mut Solution) {
        
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