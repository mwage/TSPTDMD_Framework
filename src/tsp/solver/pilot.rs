use super::Solver;
use super::PartialGreedy;

use crate::tsp::TSPInstance;
use crate::tsp::io::Logger;
use crate::tsp::Solution;

pub struct Pilot {
    beta: usize,
    best_solutions: Vec<Solution>,
    greedy: PartialGreedy
}

impl Pilot {
    pub fn new(beta: usize) -> Self {
        Pilot {
            beta,
            best_solutions: Vec::with_capacity(beta),
            greedy: PartialGreedy::new()
        }
    }
}

impl Solver for Pilot {
    fn solve(&mut self, instance: TSPInstance, logger: Logger) {
        let mut results: Vec<(u32, u32, u32, usize)> = Vec::with_capacity(instance.number_of_vertices as usize - 1);
        // self.best_solutions.push(Solution::new(&instance));
        for i in 1..instance.number_of_vertices {
            let mut candidate = Solution::new(&instance);
            candidate.add_assignment(i, 0, instance.get_vertex(0).get_weight(i));
            let (driver, value) = self.greedy.solve(&instance, candidate);
            results.push((0, i, driver, value));
            // candidate.calculate_objective_value();
            // results.push((i, candidate.objective_value));
        }
        results.sort_by(|a, b| a.3.cmp(&b.3));
        // println!("{:?}", results);
        for i in 0..self.beta {
            self.best_solutions.push(Solution::new(&instance));
            if i < results.len() {
                let res = results[i];
                let distance = instance.get_vertex(0).get_weight(res.1);
                self.best_solutions[res.0 as usize].add_assignment(res.1, res.2, distance);
            } else {    // Padding in case beta > N(0)
                let res = results[0];
                let distance = instance.get_vertex(0).get_weight(res.1);
                self.best_solutions[res.0 as usize].add_assignment(res.1, res.2, distance);
            }
        }

        for r in 1..instance.number_of_vertices - 1 {
            let mut results: Vec<(u32, u32, u32, usize)> = Vec::with_capacity(instance.number_of_vertices as usize - r as usize);
            self.best_solutions.push(Solution::new(&instance));
            for i in 0..self.best_solutions.len() {
                let solution = self.best_solutions[i];
                for vertex in solution.unassigned_vertices() {
                    let mut candidate = solution.clone();
                    candidate.add_assignment(i, 0, instance.get_vertex(0).get_weight(i));
                    let (driver, value) = self.greedy.solve(&instance, candidate);
                    results.push((0, i, driver, value));
                    // candidate.calculate_objective_value();
                    // results.push((i, candidate.objective_value));
                }
            }
            for v in 1..instance.number_of_vertices {

            }
            results.sort_by(|a, b| a.3.cmp(&b.3));
            // println!("{:?}", results);
            for i in 0..self.beta {
                let res = results[i];
                let distance = instance.get_vertex(0).get_weight(res.1);
                self.best_solutions[res.0 as usize].add_assignment(res.1, res.2, distance);
            }
        }

        // for  in  {
            
        // }



        // let (best_vertex, distance) = self.get_best_vertex(&instance, &mut unassigned_vertices);
        // self.current_solution_mut().add_assignment(best_vertex, 0, distance, false);
    }

    fn to_string(&self) -> &str {
        "Pilot"
    }
}