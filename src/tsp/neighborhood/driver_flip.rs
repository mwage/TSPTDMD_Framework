use std::rc::Rc;

use crate::rand::Rng;
use crate::tsp::Solution;
use crate::tsp::TSPInstance;
use super::NeighborhoodImpl;

pub struct DriverFlip {

}

impl DriverFlip {
    pub fn new() -> Self {
        DriverFlip {}
    }

    pub fn apply(solution: &mut Solution, assignment: usize, new_driver: u32, delta_eval: bool) {
        let old_driver = solution.get_assignment(assignment).driver();
        let vertex = solution.get_assignment(assignment).vertex();
        solution.get_assignment_mut(assignment).set_driver(new_driver);

        if !delta_eval {
            return;
        }

        let prev_vertex = solution.get_assignment(assignment - 1).vertex();
        let distance = solution.instance().get_vertex(prev_vertex).get_weight(vertex) as isize;
        solution.delta_evaluation(old_driver, distance);
        solution.delta_evaluation(new_driver, -distance);
    }
}

impl NeighborhoodImpl for DriverFlip {
    fn apply_neighborhood(&self, solution: &mut Solution) {
        let instance = solution.instance();
        let idx = rand::thread_rng().gen_range(0, instance.number_of_vertices());
        let old_driver = solution.get_assignment(idx).driver();
        let mut new_driver = old_driver;
        while new_driver == old_driver {
            new_driver = rand::thread_rng().gen_range(0, instance.number_of_drivers()) as u32;
        }
        solution.get_assignment_mut(idx).set_driver(new_driver);
    }
}

#[test]
fn test_driver_flip() {
    let instance = TSPInstance::new(1, 2, 10);
    let mut solution = Solution::new(Rc::new(instance));
    solution.add_assignment(0, 0, 10);
    assert_eq!(solution.assignments().len(), 1);
    assert_eq!(solution.get_assignment(0).driver(), 0);
    DriverFlip::new().apply_neighborhood(&mut solution);
    assert_ne!(solution.get_assignment(0).driver(), 0);
}