use std::rc::Rc;
use std::cmp;

use super::Solver;
use super::GreedySolver;

use crate::tsp::TSPInstance;
use crate::tsp::io::Logger;
use crate::tsp::Solution;

pub struct PilotSolver {
    beta: usize,
    best_solutions: Vec<Solution>,
    greedy: GreedySolver
}

impl PilotSolver {
    pub fn new(beta: usize) -> Self {
        PilotSolver {
            beta,
            best_solutions: Vec::with_capacity(beta),
            greedy: GreedySolver::new()
        }
    }
}

impl Solver for PilotSolver {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        self.greedy.set_instance(&instance);    // Initialize instance of greedy algorithm
        self.best_solutions.push(Solution::new(Rc::clone(&instance)));   // Add an initial empty solution
        let mut results: Vec<(usize, u32, u32, usize)> = Vec::with_capacity(instance.number_of_vertices() as usize - 1);  // Initialize vector for results of different branches empty
        
        for i in 0..instance.number_of_vertices() - 1 {
            for j in 0..self.best_solutions.len() {
                let solution = &self.best_solutions[j];
                for vertex in solution.unassigned_vertices() {  // Loop over all neighbors
                    let solution = self.greedy.solve_from_solution(solution.clone(), *vertex); // Use greedy algorithm to solve the branch
                    // println!("{}: {}", solution.get_assignment(i).driver(), solution.drivers_to_str());
                    results.push((j, *vertex, solution.get_assignment(i).driver(), solution.objective_value()));  // Add the result to the list
                }
            }

               // sort results by objective value
            // println!("{:?}", results);

            // Select best beta and update solution accordingly.

            let min = cmp::min(results.len(), self.beta);
            let mut new_best = Vec::new();
            for i in 0..min {
                let res = results[i];
                let mut solution = self.best_solutions[res.0].clone();
                let vertex = res.1;
                let driver = res.2;
                let distance = instance.get_vertex(solution.get_last_vertex()).get_weight(vertex);
                solution.add_assignment(vertex, driver, distance);
                new_best.push(solution);
            }
            self.best_solutions = new_best;
            results = Vec::new();   // Reset results
        }

        // Finish solution
        let solution = &mut self.best_solutions[0];
        let driver = solution.get_smallest_driver();
        let distance = instance.get_vertex(solution.get_last_vertex()).get_weight(0);    // distance between the last assigned vertex and vertex 0
        solution.add_assignment(0, driver, distance);

        // Logging
        solution.calculate_objective_value();
        println!("Val: {}", solution.objective_value());
        println!("Target: {}", instance.desired_travel_distance());
        println!("{:?}", solution.driver_distances());
        logger.log_result(solution);
    }

    fn to_string(&self) -> String {
        format!("Pilot.{}", self.beta)
    }
}