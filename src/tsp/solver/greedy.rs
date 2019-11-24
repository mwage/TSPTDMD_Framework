use std::rc::Rc;
use std::cmp;

use crate::tsp::io::Logger;
use crate::tsp::TSPInstance;
use crate::tsp::Solution;
use crate::rand::Rng;
use super::Solver;


pub struct GreedySolver {
    candidate_size: usize,
    instance: Option<Rc<TSPInstance>>
}

impl GreedySolver {
    pub fn new(candidate_size: usize) -> GreedySolver {
        GreedySolver {
            candidate_size,
            instance: None
        }
    }
    
    pub fn candidate_size(&self) -> usize {
        self.candidate_size
    }

    fn instance(&self) -> &Rc<TSPInstance> {
        if let Some(instance) = &self.instance {
            instance
        } else {
            panic!("Tried accessing uninitialized instance.");
        }
    }

    fn calculate_target_distance(&self, solution: &Solution) -> isize {
        // Calculate available capacity as the sum of the missing distances
        let available_capacity = solution.driver_distances().iter()
            .filter(|x| **x < self.instance().desired_travel_distance())
            .fold(0, |acc, x| acc + self.instance().desired_travel_distance() - *x);

        // Return total capacity / number of unvisited vertices
        available_capacity / (self.instance().number_of_vertices() - solution.number_of_assignments()) as isize   
    }

    fn get_best_vertex(&self, solution: &Solution) -> (usize, isize) {
        let target_distance = self.calculate_target_distance(solution); // Total available capacity / unvisited vertices
        let last_vertex = solution.get_last_vertex();
        let best_vertex = *(solution.unassigned_vertices().iter()  // Find vertex who's distance is closest to target distance
            .min_by_key(|x| (self.instance().get_vertex(last_vertex).get_weight(**x) - target_distance).abs()).unwrap());

        (best_vertex, self.instance().get_vertex(last_vertex).get_weight(best_vertex))
    }
    
    fn get_random_best_vertex(&self, solution: &Solution) -> (usize, isize) {
        let target_distance = self.calculate_target_distance(solution); // Total available capacity / unvisited vertices
        let last_vertex = solution.get_last_vertex();

        // Calculate the deviation from the target distance for all unassinged vertices
        let mut differences: Vec<(usize, isize)> = solution.unassigned_vertices().iter()  
            .map(|i| (*i, (self.instance().get_vertex(last_vertex).get_weight(*i) - target_distance).abs())).collect();
        differences.sort_by(|a, b| a.1.cmp(&b.1));
        let vertex = differences[rand::thread_rng().gen_range(0, cmp::min(differences.len(), self.candidate_size))].0;

        (vertex, self.instance().get_vertex(last_vertex).get_weight(vertex))
    }

    pub fn set_instance(&mut self, instance: &Rc<TSPInstance>) {
        self.instance = Some(Rc::clone(instance));  // Initialize tsp instance
    }

    pub fn solve_from_solution(&mut self, solution: &mut Solution, next_vertex: usize, logger: &Logger) {
        let next_driver = solution.get_smallest_driver();    // Find best driver
        let distance = self.instance().get_vertex(solution.get_last_vertex()).get_weight(next_vertex);  // Get distance between last vertex and chosen next vertex
        solution.add_assignment(next_vertex, next_driver, distance); // Add the chosen vertex with the assigned driver to the solution
        self.solve_greedy(solution, logger);    // Solve the instance
        solution.calculate_objective_value();    // Calculate the objective value of the solution
    }

    pub fn solve_greedy(&mut self, solution: &mut Solution, logger: &Logger) {
        while !solution.is_complete() {  // Loop until only the home trip is left
            let next_driver = solution.get_smallest_driver();    // Find best driver
            let (best_vertex, distance) = if self.candidate_size > 1 {    // Find next best vertex
                self.get_random_best_vertex(solution) 
            } else { 
                self.get_best_vertex(solution)
            };
            solution.add_assignment(best_vertex, next_driver, distance); // Add new vertex to the solution

            if logger.get_elapsed() >= crate::TIME_LIMIT {
                return;
            }
        }
        let next_driver = solution.get_smallest_driver();    // Find best driver
        let distance = self.instance().get_vertex(solution.get_last_vertex()).get_weight(0);    // distance between the last assigned vertex and vertex 0
        solution.add_assignment(0, next_driver, distance);   // Add final return to vertex 0
    }
}

impl Solver for GreedySolver {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {        
        self.instance = Some(Rc::clone(&instance)); // Initialize TSP instance
        let mut solution = Solution::new(Rc::clone(&instance));  // Initialize solution

        self.solve_greedy(&mut solution, &logger);    // Solve the remaining problem

        // Logging
        solution.calculate_objective_value();    // Calculate the objective value of the solution
        println!("Val: {}", solution.objective_value());
        println!("Target: {}", self.instance().desired_travel_distance());
        println!("{:?}", solution.driver_distances());

        logger.log_result(&solution);    // Log results
    }

    fn to_string(&self) -> String {
        format!("Greedy.{}", self.candidate_size)
    }
}