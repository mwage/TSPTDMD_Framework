use std::rc::Rc;
use std::cmp;

use super::Solver;
use super::GreedySolver;

use crate::tsp::TSPInstance;
use crate::tsp::io::Logger;
use crate::tsp::Solution;

pub struct PilotSolver {
    beta: usize,
    greedy: GreedySolver
}

impl PilotSolver {
    pub fn new(beta: usize) -> Self {
        PilotSolver {
            beta,
            greedy: GreedySolver::new(1)
        }
    }
}

impl Solver for PilotSolver {
    fn solve(&mut self, instance: Rc<TSPInstance>, logger: Logger) {
        let mut best_solutions = Vec::with_capacity(self.beta);
        best_solutions.push(Solution::new(Rc::clone(&instance)));   // Add an initial empty solution
        let mut results: Vec<(usize, usize, usize, i128)> = Vec::with_capacity(instance.number_of_vertices() - 1);  // Initialize vector for results of different branches empty
        
        for i in 0..instance.number_of_vertices() - 1 {
            for j in 0..best_solutions.len() {
                let solution = &best_solutions[j];
                for vertex in solution.unassigned_vertices() {  // Loop over all neighbors
                    let mut solution = solution.clone();
                    self.greedy.solve_from_solution(&instance, &mut solution, *vertex, &logger); // Use greedy algorithm to solve the branch
                    results.push((j, *vertex, solution.get_assignment(i).driver(), solution.objective_value()));  // Add the result to the list

                    if logger.get_elapsed() >= crate::TIME_LIMIT {  // Time termination
                        logger.log_result(&solution);
                        return;
                    }
                }
            }

            // sort results by objective value
            results.sort_by(|a, b| a.3.cmp(&b.3));  // sort results by objective value

            // Select best beta and update solution accordingly.
            let min = cmp::min(results.len(), self.beta);
            let mut new_best = Vec::new();
            for i in 0..min {
                let res = results[i];
                let mut solution = best_solutions[res.0].clone();
                let vertex = res.1;
                let driver = res.2;
                let distance = instance.get_vertex(solution.get_last_vertex()).get_weight(vertex);
                solution.add_assignment(vertex, driver, distance);
                new_best.push(solution);
            }
            best_solutions = new_best;
            results = Vec::new();   // Reset results vector
        }

        // Finish solution
        let solution = &mut best_solutions[0];
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
        format!("pilot.{}", self.beta)
    }
}