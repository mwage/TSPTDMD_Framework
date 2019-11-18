use super::Solver;

use crate::tsp::io::Logger;
use crate::tsp::TSPInstance;
use crate::tsp::Solution;

use std::usize;
use std::u32;


pub struct GreedySolver {
    current_solution: Option<Solution>
}

impl GreedySolver {
    pub fn new() -> GreedySolver {
        GreedySolver {
            current_solution: None
        }
    }
    
    fn get_next_driver(&self) -> u32 {
        let mut min_distance = usize::max_value();        
        let mut best_driver = u32::max_value();
        let solution = self.current_solution();
        for i in 0..solution.drivers() {
            let distance = solution.get_driver_distance(i);
            if distance < min_distance {
                min_distance = distance;
                best_driver = i as u32;
            }
        }

        best_driver
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

    fn calculate_target_distance(&self) -> usize {
        let mut available_capacity = 0;
        for distance in self.current_solution().driver_distances() {    // Sum of available capacity
            if *distance < self.current_solution().desired_travel_distance {
                available_capacity += self.current_solution().desired_travel_distance - *distance;
            }
        }
        available_capacity / (self.current_solution().number_of_vertices - self.current_solution().assignments())
    }

    fn get_best_vertex(&self, instance: &TSPInstance, unassigned_vertices: &mut Vec<u32>) -> (u32, usize) {
        let mut min_difference = usize::max_value();
        let mut best_vertex = 0;
        let target_distance = self.calculate_target_distance(); // Total available capacity / unvisited vertices

        for i in unassigned_vertices.iter() {
            let difference = (instance.get_vertex(0).get_weight(*i) as isize - target_distance as isize).abs() as usize;
            if difference < min_difference {
                min_difference = difference;
                best_vertex = *i;
            }
        }

        let idx = unassigned_vertices.iter().position(|x| *x == best_vertex).unwrap();  // Remove vertex out of unassigned
        unassigned_vertices.remove(idx);
        (best_vertex, min_difference)
    }
}

impl Solver for GreedySolver {
    fn solve(&mut self, instance: TSPInstance, logger: Logger) {
        self.current_solution = Some(Solution::new(&instance));
        let mut unassigned_vertices: Vec<u32> = (1..instance.number_of_vertices).collect();
        let (best_vertex, distance) = self.get_best_vertex(&instance, &mut unassigned_vertices);
        self.current_solution_mut().add_assignment(best_vertex, 0, distance, false);
        // self.current_solution().print();
        // println!("Target: {}", self.current_solution().desired_travel_distance);
        // println!("{:?}", self.current_solution().driver_distances());

        let mut last_vertex = best_vertex;
        while !self.current_solution().is_complete() {
            let next_driver = self.get_next_driver();
            let (best_vertex, distance) = self.get_best_vertex(&instance, &mut unassigned_vertices);
            self.current_solution_mut().add_assignment(best_vertex, next_driver, distance, false);
            last_vertex = best_vertex;
            // self.current_solution().print();
            // println!("{:?}", self.current_solution().driver_distances());
        }
        let next_driver = self.get_next_driver();
        let distance = instance.get_vertex(last_vertex as usize).get_weight(0);
        self.current_solution_mut().add_assignment(0, next_driver, distance, false);
        // self.current_solution().print();
        // println!("{:?}", self.current_solution().driver_distances());
        logger.log_result(&self.current_solution());
    }

    fn to_string(&self) -> &str {
        "Greedy"
    }
}