use super::Solver;

use crate::tsp::io::Logger;
use crate::tsp::TSPInstance;
use crate::tsp::Solution;

use std::rc::Rc;

pub struct GreedySolver {
    current_solution: Option<Solution>,
    instance: Option<Rc<TSPInstance>>
}

impl GreedySolver {
    pub fn new() -> GreedySolver {
        GreedySolver {
            current_solution: None,
            instance: None
        }
    }

    fn current_solution(&self) -> &Solution {
        if let Some(solution) = &self.current_solution {
            solution
        } else {
            panic!("Tried accessing uninitialized solution.");
        }
    }

    fn current_solution_mut(&mut self) -> &mut Solution {
        if let Some(solution) = &mut self.current_solution {
            solution
        } else {
            panic!("Tried accessing uninitialized solution.");
        }
    }

    fn instance(&self) -> &Rc<TSPInstance> {
        if let Some(instance) = &self.instance {
            instance
        } else {
            panic!("Tried accessing uninitialized solution.");
        }
    }

    fn calculate_target_distance(&self) -> usize {
        let mut available_capacity = 0;
        for distance in self.current_solution().driver_distances() {    // Sum of available capacity
            if *distance < self.instance().desired_travel_distance() {
                available_capacity += self.instance().desired_travel_distance() - *distance;
            }
        }
        available_capacity / (self.instance().number_of_vertices() - self.current_solution().number_of_assignments())
    }

    fn get_best_vertex(&self) -> (u32, usize) {
        let mut min_difference = usize::max_value();
        let mut best_vertex = 0;
        let target_distance = self.calculate_target_distance(); // Total available capacity / unvisited vertices

        for i in self.current_solution().unassigned_vertices() {   // Find vertex closest to the target distance
            let difference = (self.instance().get_vertex(0).get_weight(*i) as isize - target_distance as isize).abs() as usize;
            if difference < min_difference {
                min_difference = difference;
                best_vertex = *i;
            }
        }

        (best_vertex, min_difference)
    }
}

impl Solver for GreedySolver {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        self.instance = Some(Rc::clone(&instance));
        self.current_solution = Some(Solution::new(Rc::clone(&instance)));
        let (best_vertex, distance) = self.get_best_vertex();
        self.current_solution_mut().add_assignment(best_vertex, 0, distance);
        // self.current_solution().print();
        // println!("Target: {}", self.current_solution().desired_travel_distance);
        // println!("{:?}", self.current_solution().driver_distances());

        let mut last_vertex = best_vertex;
        while !self.current_solution().is_complete() {
            let next_driver = self.current_solution().get_smallest_driver();
            let (best_vertex, distance) = self.get_best_vertex();
            self.current_solution_mut().add_assignment(best_vertex, next_driver, distance);
            last_vertex = best_vertex;
        }
        let next_driver = self.current_solution().get_smallest_driver();
        let distance = instance.get_vertex(last_vertex as usize).get_weight(0);
        self.current_solution_mut().add_assignment(0, next_driver, distance);
        logger.log_result(&self.current_solution());
    }

    fn to_string(&self) -> &str {
        "Greedy"
    }
}