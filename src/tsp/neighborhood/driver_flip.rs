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