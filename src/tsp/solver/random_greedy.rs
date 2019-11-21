use std::rc::Rc;
use std::cmp;

use crate::tsp::io::Logger;
use crate::tsp::TSPInstance;
use crate::tsp::Solution;
use crate::rand::Rng;
use super::Solver;


pub struct RandomGreedySolver {
    candidate_size: usize,
    current_solution: Option<Solution>,
    instance: Option<Rc<TSPInstance>>
}

impl RandomGreedySolver {
    pub fn new(candidate_size: usize) -> RandomGreedySolver {
        RandomGreedySolver {
            candidate_size,
            current_solution: None,
            instance: None
        }
    }

    pub fn current_solution(&self) -> &Solution {
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
        for distance in self.current_solution().driver_distances() {    // Calculate the sum of available capacity
            if *distance < self.instance().desired_travel_distance() {
                available_capacity += self.instance().desired_travel_distance() - *distance;
            }
        }
        available_capacity / (self.instance().number_of_vertices() - self.current_solution().number_of_assignments())   // Return total capacity / number of unvisited vertices
    }

    fn get_random_best_vertex(&self) -> (u32, usize) {
        let mut differences = Vec::new();
        let target_distance = self.calculate_target_distance(); // Total available capacity / unvisited vertices
        let last_vertex = self.current_solution().get_last_vertex();
        for i in self.current_solution().unassigned_vertices() {   // Find vertex closest to the target distance
            differences.push((i, (self.instance().get_vertex(last_vertex).get_weight(*i) as isize - target_distance as isize).abs() as usize)); 
        }
        differences.sort_by(|a, b| a.1.cmp(&b.1));
        let vertex = *differences[rand::thread_rng().gen_range(0, cmp::min(differences.len(), self.candidate_size))].0;
        (vertex, self.instance().get_vertex(last_vertex).get_weight(vertex))
    }

    fn solve_greedy(&mut self, logger: &Logger) {
        while !self.current_solution().is_complete() {  // Loop until only the home trip is left
            let next_driver = self.current_solution().get_smallest_driver();    // Find best driver
            let (best_vertex, distance) = self.get_random_best_vertex();   // Find next best vertex
            self.current_solution_mut().add_assignment(best_vertex, next_driver, distance); // Add new vertex to the solution

            if logger.get_elapsed() >= crate::TIME_LIMIT {
                return;
            }
        }
        let next_driver = self.current_solution().get_smallest_driver();    // Find best driver
        let distance = self.instance().get_vertex(self.current_solution().get_last_vertex()).get_weight(0);    // distance between the last assigned vertex and vertex 0
        self.current_solution_mut().add_assignment(0, next_driver, distance);   // Add final return to vertex 0
    }
}

impl Solver for RandomGreedySolver {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {        
        self.instance = Some(Rc::clone(&instance)); // Initialize TSP instance
        self.current_solution = Some(Solution::new(Rc::clone(&instance)));  // Initialize solution

        self.solve_greedy(&logger);    // Solve the remaining problem

        // Logging
        self.current_solution_mut().calculate_objective_value();    // Calculate the objective value of the solution
        println!("Val: {}", self.current_solution().objective_value());
        println!("Target: {}", self.instance().desired_travel_distance());
        println!("{:?}", self.current_solution().driver_distances());

        logger.log_result(&self.current_solution());    // Log results
    }

    fn to_string(&self) -> String {
        format!("RandomGreedy.{}", self.candidate_size)
    }
}