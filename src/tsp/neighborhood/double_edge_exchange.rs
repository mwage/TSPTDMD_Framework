use std::rc::Rc;

use super::NeighborhoodImpl;
use crate::tsp::Solution;
use crate::tsp::TSPInstance;

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
        let number_of_vertices = solution.instance().number_of_vertices();
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

        let prev_vertex = solution.get_assignment((start_idx - 1) % number_of_vertices).vertex();
        let old_distance = solution.instance().get_vertex(prev_vertex).get_weight(copy[0].vertex()) as isize;   // Old distance of d0 to start vertex
        let new_vertex = solution.get_assignment(start_idx).vertex();
        let new_distance = solution.instance().get_vertex(prev_vertex).get_weight(new_vertex) as isize;
        solution.delta_evaluation(solution.get_assignment(start_idx).driver(), old_distance - new_distance);

        let next_vertex = solution.get_assignment((start_idx + length + 1) % number_of_vertices).vertex();
        let old_distance = solution.instance().get_vertex(next_vertex).get_weight(copy[copy.len() - 1].vertex()) as isize;   // Old distance of d0 to start vertex
        let new_vertex = solution.get_assignment(start_idx + length).vertex();
        let new_distance = solution.instance().get_vertex(next_vertex).get_weight(new_vertex) as isize;
        solution.delta_evaluation(solution.get_assignment(start_idx + length + 1).driver(), old_distance - new_distance);
    }
}

impl NeighborhoodImpl for DoubleEdgeExchange {
    fn get_random_neighbor(&self, solution: &mut Solution) {

    }
    fn get_best_improving_neighbor(&self, solution: &mut Solution) {

    }
}

#[test]
fn test_double_edge_thingy() {
    let neighborhood = DoubleEdgeExchange::new(0);
    let vertices = 5;
    let instance = TSPInstance::new(vertices, vertices, 10);
    let mut solution = Solution::new(Rc::new(instance));
    for i in 0..vertices as u32 {
        solution.add_assignment(i, i, 10);
        assert_eq!(solution.get_assignment(i as usize).driver(), i);
        assert_eq!(solution.get_assignment(i as usize).vertex(), i);
    }
    assert_eq!(solution.assignments().len(), vertices);
    DoubleEdgeExchange::apply(&mut solution, 1, 2, false);
    assert_eq!(solution.get_assignment(0).vertex(), 0);
    assert_eq!(solution.get_assignment(0).driver(), 0);
    assert_eq!(solution.get_assignment(1).vertex(), 3);
    assert_eq!(solution.get_assignment(1).driver(), 1);
    assert_eq!(solution.get_assignment(2).vertex(), 2);
    assert_eq!(solution.get_assignment(2).driver(), 3);
    assert_eq!(solution.get_assignment(3).vertex(), 1);
    assert_eq!(solution.get_assignment(3).driver(), 2);
    assert_eq!(solution.get_assignment(4).vertex(), 4);
    assert_eq!(solution.get_assignment(4).driver(), 4);

    // Test overflow
    let instance = TSPInstance::new(vertices, vertices, 10);
    let mut solution = Solution::new(Rc::new(instance));
    for i in 0..vertices as u32 {
        solution.add_assignment(i, i, 10);
        assert_eq!(solution.get_assignment(i as usize).driver(), i);
        assert_eq!(solution.get_assignment(i as usize).vertex(), i);
    }
    assert_eq!(solution.assignments().len(), vertices);
    DoubleEdgeExchange::apply(&mut solution, 3, 2, false);
    assert_eq!(solution.get_assignment(0).vertex(), 3);
    assert_eq!(solution.get_assignment(0).driver(), 4);
    assert_eq!(solution.get_assignment(1).vertex(), 1);
    assert_eq!(solution.get_assignment(1).driver(), 1);
    assert_eq!(solution.get_assignment(2).vertex(), 2);
    assert_eq!(solution.get_assignment(2).driver(), 2);
    assert_eq!(solution.get_assignment(3).vertex(), 0);
    assert_eq!(solution.get_assignment(3).driver(), 3);
    assert_eq!(solution.get_assignment(4).vertex(), 4);
    assert_eq!(solution.get_assignment(4).driver(), 0);
}

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
    for i in 0..vertices as u32 {
        solution.add_assignment(i, i, 10);
        assert_eq!(solution.get_assignment(i as usize).driver(), i);
        assert_eq!(solution.get_assignment(i as usize).vertex(), i);
    }
    solution.calculate_objective_value();
    DoubleEdgeExchange::apply(&mut solution, 1, 2, true);
    let x = solution.objective_value();
    solution.calculate_objective_value();
    assert_eq!(x, solution.objective_value());
}