use super::Solver;

use crate::tsp::io::Logger;
use crate::tsp::TSPInstance;
use crate::tsp::Solution;
use crate::tsp::instance::Assignment;

use std::usize;
use std::u32;


pub struct PartialGreedy {
    current_solution: Option<Solution>
}

impl PartialGreedy {
    pub fn new() -> PartialGreedy {
        PartialGreedy {
            current_solution: None
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

    fn calculate_target_distance(&self) -> usize {
        let mut available_capacity = 0;
        for distance in self.current_solution().driver_distances() {    // Sum of available capacity
            if *distance < self.current_solution().desired_travel_distance {
                available_capacity += self.current_solution().desired_travel_distance - *distance;
            }
        }
        available_capacity / (self.current_solution().number_of_vertices - self.current_solution().number_of_assignments())
    }

    fn get_best_vertex(&self, instance: &TSPInstance, unassigned_vertices: &mut Vec<u32>) -> (u32, usize) {
        let mut min_difference = usize::max_value();
        let mut best_vertex = 0;
        let target_distance = self.calculate_target_distance(); // Total available capacity / unvisited vertices

        for i in unassigned_vertices.iter() {   // Find vertex closest to the target distance
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
    
    pub fn solve(&mut self, instance: &TSPInstance, solution: Solution, vertex: u32) -> (u32, usize) {
        self.current_solution = Some(solution);
        let mut unassigned_vertices: Vec<u32> = (1..instance.number_of_vertices).collect();
        for assignment in self.current_solution().assignments() {
            let idx = unassigned_vertices.iter().position(|x| *x == assignment.vertex).unwrap();  // Remove vertex out of unassigned
            unassigned_vertices.remove(idx);
        }

        let driver = self.current_solution().get_smallest_driver();  // Find driver for first vertex
        let distance = instance.get_vertex(self.current_solution().assignments().last().unwrap().vertex as usize).get_weight(vertex);
        self.current_solution_mut().add_assignment(vertex, 0, distance);

        // self.current_solution().print();
        // println!("Target: {}", self.current_solution().desired_travel_distance);
        // println!("{:?}", self.current_solution().driver_distances());

        let mut last_vertex = self.current_solution().assignments().last().unwrap().vertex;
        let mut first = true;
        let mut driver = 0;

        while !self.current_solution().is_complete() {
            let next_driver = self.current_solution().get_smallest_driver();
            if first {
                driver = next_driver;
                first = false;
            }
            let (best_vertex, distance) = self.get_best_vertex(&instance, &mut unassigned_vertices);
            self.current_solution_mut().add_assignment(best_vertex, next_driver, distance);
            last_vertex = best_vertex;
            // self.current_solution().print();
            // println!("{:?}", self.current_solution().driver_distances());
        }
        let next_driver = self.current_solution().get_smallest_driver();
        let distance = instance.get_vertex(last_vertex as usize).get_weight(0);
        self.current_solution_mut().add_assignment(0, next_driver, distance);
        // self.current_solution().print();
        // println!("{:?}", self.current_solution().driver_distances());
        self.current_solution_mut().calculate_objective_value();
        (driver, self.current_solution().objective_value)
    }
}