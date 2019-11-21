use std::rc::Rc;

use crate::rand::Rng;
use crate::tsp::Solution;
use crate::tsp::TSPInstance;
use super::NeighborhoodImpl;

pub struct DriverSwap {

}

impl DriverSwap {
    pub fn new() -> Self {
        DriverSwap {}
    }

    fn apply(&self, solution: &mut Solution, first_assignment: usize, second_assignment: usize) {
        let instance = solution.instance();
        let idx = rand::thread_rng().gen_range(0, instance.number_of_vertices());


        let first_driver = solution.get_assignment(first_assignment).driver();
        let second_driver = solution.get_assignment(second_assignment).driver();

        solution.get_assignment_mut(first_assignment).set_driver(second_driver);
        solution.get_assignment_mut(second_assignment).set_driver(first_driver);
    }

    fn get_random_idx() {
        // let mut second_idx = idx;
        // while second_idx == idx {
        //     second_idx = rand::thread_rng().gen_range(0, instance.number_of_vertices());
        // }
    }
}

impl NeighborhoodImpl for DriverSwap {
    fn apply_neighborhood(&self, solution: &mut Solution) {

    }
}

#[test]
fn test_driver_swap() {
    let instance = TSPInstance::new(2, 2, 10);
    let mut solution = Solution::new(Rc::new(instance));
    solution.add_assignment(0, 0, 10);
    solution.add_assignment(1, 1, 10);
    assert_eq!(solution.assignments().len(), 2);
    assert_eq!(solution.get_assignment(0).driver(), 0);
    assert_eq!(solution.get_assignment(1).driver(), 1);
    DriverSwap::new().apply_neighborhood(&mut solution);
    assert_eq!(solution.get_assignment(0).driver(), 1);
    assert_eq!(solution.get_assignment(1).driver(), 0);
}