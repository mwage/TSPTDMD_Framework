use std::rc::Rc;

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
        let mut results: Vec<(u32, u32, u32, usize)> = Vec::with_capacity(instance.number_of_vertices() as usize - 1);  // Initialize vector with results of the different branches
        for i in 1..instance.number_of_vertices() as u32 {  // Loop over all neighbors
            let solution = self.greedy.solve_from_solution(Solution::new(Rc::clone(&instance)), i); // Use greedy algorithm to solve the branch
            results.push((0, i, solution.get_assignment(0).driver(), solution.objective_value()));  // Add the result to the list

            // candidate.calculate_objective_value();
            // results.push((i, candidate.objective_value));
        }
        results.sort_by(|a, b| a.3.cmp(&b.3));
        println!("{:?}", results);

        // for i in 0..self.beta {
        //     self.best_solutions.push(Solution::new(Rc::clone(&instance)));
        //     if i < results.len() {
        //         let res = results[i];
        //         let distance = instance.get_vertex(0).get_weight(res.1);
        //         self.best_solutions[res.0 as usize].add_assignment(res.1, res.2, distance);
        //     } else {    // Padding in case beta > N(0)
        //         let res = results[0];
        //         let distance = instance.get_vertex(0).get_weight(res.1);
        //         self.best_solutions[res.0 as usize].add_assignment(res.1, res.2, distance);
        //     }
        // }

        // for r in 1..instance.number_of_vertices() - 1 {
        //     let mut results: Vec<(u32, u32, u32, usize)> = Vec::with_capacity(instance.number_of_vertices() - r);
        //     self.best_solutions.push(Solution::new(Rc::clone(&instance)));
        //     for i in 0..self.best_solutions.len() as u32 {
        //         let solution = &self.best_solutions[i as usize];
        //         for vertex in solution.unassigned_vertices() {
        //             let mut candidate = solution.clone();
        //             let solution = self.greedy.solve_from_solution(candidate, i);
        //             results.push((0, i, solution.get_assignment(0).driver(), solution.objective_value()));
        //             // candidate.calculate_objective_value();
        //             // results.push((i, candidate.objective_value));
        //         }
        //     }
        //     for v in 1..instance.number_of_vertices() {

        //     }
        //     results.sort_by(|a, b| a.3.cmp(&b.3));
        //     // println!("{:?}", results);
        //     for i in 0..self.beta {
        //         let res = results[i];
        //         let distance = instance.get_vertex(0).get_weight(res.1);
        //         self.best_solutions[res.0 as usize].add_assignment(res.1, res.2, distance);
        //     }
        // }

        let solution = self.greedy.solve_from_solution(Solution::new(Rc::clone(&instance)), 4);
        println!("{}", instance.desired_travel_distance());
        println!("{:?}", solution.driver_distances());
        logger.log_result(&solution);
        // for  in  {
            
        // }



        // let (best_vertex, distance) = self.get_best_vertex(&instance, &mut unassigned_vertices);
        // self.current_solution_mut().add_assignment(best_vertex, 0, distance, false);
    }

    fn to_string(&self) -> &str {
        "Pilot"
    }
}