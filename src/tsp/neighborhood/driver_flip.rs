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

    pub fn get_delta(solution: &Solution, assignment: usize, new_driver: u32) -> isize {
        let distance = solution.get_distance(assignment) as isize;
        let old_driver_distance = solution.get_driver_distance(solution.get_assignment(assignment).driver() as usize) as isize - distance;
        let new_driver_distance = solution.get_driver_distance(new_driver as usize) as isize;
        2 * distance * (new_driver_distance - old_driver_distance)
    }
}

impl NeighborhoodImpl for DriverFlip {
    fn get_random_neighbor(&self, solution: &mut Solution, delta_eval: bool) {
        let instance = solution.instance();
        let idx = rand::thread_rng().gen_range(0, instance.number_of_vertices());
        let old_driver = solution.get_assignment(idx).driver();
        let mut new_driver = old_driver;
        while new_driver == old_driver {
            new_driver = rand::thread_rng().gen_range(0, instance.number_of_drivers()) as u32;
        }
        DriverFlip::apply(solution, idx, new_driver, delta_eval);
    }

    fn get_best_improving_neighbor(&self, solution: &mut Solution, delta_eval: bool) {

    }    

    fn get_first_improving_neighbor(&self, solution: &mut Solution, delta_eval: bool) {
        
    }

    fn to_string(&self) -> String {
        String::from("DriverFlip")
    }
}

#[test]
fn test_driver_flip() {
    let instance = TSPInstance::new(1, 2, 10);
    let mut solution = Solution::new(Rc::new(instance));
    solution.add_assignment(0, 0, 10);
    assert_eq!(solution.assignments().len(), 1);
    assert_eq!(solution.get_assignment(0).driver(), 0);
    DriverFlip::apply(&mut solution, 0, 1, false);
    assert_eq!(solution.get_assignment(0).driver(), 1);
}

#[test]
fn test_delta() {
    let instance = TSPInstance::new_random(10, 3, 100, 50);
    let mut solution = Solution::new_random(Rc::new(instance));
    solution.calculate_objective_value();
    let idx = rand::thread_rng().gen_range(0, 10);
    let old_driver = solution.get_assignment(idx).driver();
    let new_driver = (old_driver + 1) % 3;
    let new_val = DriverFlip::get_delta(&solution,idx, new_driver) + solution.objective_value() as isize;
    DriverFlip::apply(&mut solution, idx, new_driver, true);
    assert_eq!(new_val, solution.objective_value() as isize);
}