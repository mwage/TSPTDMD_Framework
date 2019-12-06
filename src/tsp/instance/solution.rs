use std::rc::Rc;
use std::cmp;
use rand::prelude::*;

use crate::tsp::TSPInstance;
use super::Assignment;
use crate::modulo_pos;

#[derive(Clone, Debug)]
pub struct Solution {
    assignments: Vec<Assignment>,
    unassigned_vertices: Vec<usize>,
    driver_distances: Vec<i64>,
    instance: Rc<TSPInstance>,
    objective_value: i128
}

impl Solution {
    pub fn new(instance: Rc<TSPInstance>) -> Self {
        Solution {
            assignments: Vec::with_capacity(instance.number_of_vertices()),
            unassigned_vertices: (1..instance.number_of_vertices()).collect(),
            driver_distances: vec![0; instance.number_of_drivers()],
            instance,
            objective_value: i128::max_value()
        }
    }

    pub fn new_random(instance: Rc<TSPInstance>) -> Self {
        let mut rng = rand::thread_rng();
        let mut vertices: Vec<usize> = (1..instance.number_of_vertices()).collect();
        let mut solution = Solution::new(instance);
        vertices.shuffle(&mut rng);
        for vertex in vertices.iter() {
            let driver = rand::thread_rng().gen_range(0, solution.instance().number_of_drivers());
            let last_vertex = solution.get_last_vertex();
            solution.add_assignment(*vertex, driver, solution.instance().get_vertex(*vertex).get_weight(last_vertex));
        }
        let driver = rand::thread_rng().gen_range(0, solution.instance().number_of_drivers());
        let last_vertex = solution.get_last_vertex();
        solution.add_assignment(0, driver, solution.instance().get_vertex(0).get_weight(last_vertex));
        solution
    }

    pub fn unassigned_vertices(&self) -> &Vec<usize> {
        &self.unassigned_vertices
    }

    pub fn objective_value(&self) -> i128 {
        self.objective_value
    }

    pub fn driver_distances(&self) -> &Vec<i64> {
        &self.driver_distances
    }

    pub fn get_driver_distance(&self, idx: usize) -> i64 {
        self.driver_distances[idx]
    }

    pub fn get_assignment(&self, idx: usize) -> &Assignment {
        &self.assignments[idx]
    }

    pub fn get_assignment_mut(&mut self, idx: usize) -> &mut Assignment {
        &mut self.assignments[idx]
    }

    pub fn number_of_assignments(&self) -> usize {
        self.assignments.len()
    }

    pub fn is_complete(&self) -> bool {
        self.assignments.len() >= self.instance.number_of_vertices() - 1
    }

    pub fn instance(&self) -> &Rc<TSPInstance> {
        &self.instance
    }

    pub fn assignments(&mut self) -> &mut Vec<Assignment> {
        &mut self.assignments
    }

    pub fn delta_evaluation(&mut self, delta: i128, distances: Vec<i64>) {
        self.objective_value += delta;
        self.driver_distances = distances;
    }

    pub fn get_distance(&self, idx: usize) -> i64 {
        let first = self.get_assignment(idx).vertex();
        let prev = self.get_assignment(modulo_pos(idx as isize - 1, self.instance.number_of_vertices())).vertex();
        self.instance.get_vertex(first).get_weight(prev)
    }
    
    pub fn add_assignment(&mut self, vertex: usize, driver: usize, distance: i64) {
        if self.assignments.len() > self.instance.number_of_vertices() {
            panic!("Exceeded maximum number of assignments.");
        }
        self.assignments.push(Assignment::new(vertex, driver));        
        self.driver_distances[driver] += distance;

        if vertex != 0 {
            let idx = self.unassigned_vertices.iter().position(|x| *x == vertex).unwrap();  // Remove vertex out of unassigned
            self.unassigned_vertices.remove(idx);
        }
    }

    pub fn get_last_vertex(&self) -> usize {
        match self.assignments.last() {
            Some(x) => x.vertex(),
            None => 0
        }
    }

    // Get driver with smallest distance
    pub fn get_smallest_driver(&self) -> usize {  
        let (best_driver, _) = self.driver_distances.iter().enumerate().min_by_key(|(_, x)| *x).unwrap();
        best_driver
    }
    
    // Calculates objective value from driver distances
    pub fn calculate_objective_value(&mut self) {
        self.objective_value = self.driver_distances.iter()
            .map(|x| ((self.instance.desired_travel_distance() - *x) as i128).pow(2))
            .collect::<Vec<i128>>().iter().sum();
    }
    
    // Calculates objective value by recalculating driver distances
    pub fn calculate_objective_value_from_scratch(&mut self) {
        self.driver_distances = vec![0; self.instance.number_of_drivers()];
        for i in 0..self.assignments.len() {
            let assignment = &self.assignments[i];
            let prev = &self.assignments[modulo_pos(i as isize - 1, self.assignments.len())];
            self.driver_distances[assignment.driver()] += self.instance.get_vertex(assignment.vertex()).get_weight(prev.vertex());
        }
        self.calculate_objective_value();
    }
    
    pub fn is_feasible(&self) -> String {
        if self.assignments.len() < self.instance.number_of_vertices() {
            return String::from("INFEASIBLE");
        }

        if self.instance.has_only_feasible_edges() {
            return String::from("FEASIBLE")
        }

        let mut feasible = true;
        for i in 0..self.assignments.len() {
            let first_vertex = self.assignments[i].vertex();
            let second_vertex = self.assignments[(i + 1) % self.assignments.len()].vertex();
            if !self.instance.is_valid(first_vertex, second_vertex) {
                feasible = false;
            }
        }
        if feasible {
            String::from("FEASIBLE")
        } else {
            String::from("INFEASIBLE")
        }
    }

    pub fn vertices_to_str(&self) -> String {
        let mut result = String::new();
        let number_of_vertices = self.instance.number_of_vertices();
        
        // If solution is a permutation with 0 not at the end: permute vertices to end with 0
        if let Some(zero_idx) = self.assignments.iter().position(|x| x.vertex() == 0) {
            for i in 0..number_of_vertices {
                let idx = (i + zero_idx) % number_of_vertices;
                result += &self.assignments[idx].vertex().to_string();
                result.push_str(" ");
            }
            return result;
        }
        
        let min = cmp::min(number_of_vertices - 1, self.number_of_assignments());
        result += "0";

        // Otherwise just add vertices as is
        for i in 0..min {
            result.push_str(" ");
            result += &self.assignments[i].vertex().to_string();
        }
        result
    }

    pub fn drivers_to_str(&self) -> String {
        let mut result = String::new();
        let number_of_vertices = self.instance.number_of_vertices();
        
        // If solution is a permutation with 0 not at the end: permute vertices to end with 0
        if let Some(zero_idx) = self.assignments.iter().position(|x| x.vertex() == 0) {
            for i in 0..number_of_vertices {
                let idx = (i + zero_idx + 1) % number_of_vertices;
                result.push_str(" ");
                result += &self.assignments[idx].driver().to_string();
            }
            return result;
        }

        // Otherwise just add vertices as is
        for i in 0..self.assignments.len() {
            result.push_str(" ");
            result += &self.assignments[i].driver().to_string();
        }
        result
    }

    pub fn print_assignments(&self) {
        println!("{:?}", self.assignments);
    }
}

#[test]
fn test_obj_function() {
    let mut instance = TSPInstance::new(3, 2, 5);
    instance.add_edge(0, 1, 2);
    instance.add_edge(1, 2, 2);
    instance.add_edge(2, 0, 1);

    let mut solution = Solution::new(Rc::new(instance));
    solution.calculate_objective_value();
    assert_eq!(solution.objective_value, 50);
    solution.add_assignment(1, 0, 2);
    solution.calculate_objective_value();
    assert_eq!(solution.objective_value, 34);
}
