use std::rc::Rc;
use std::cmp;
use rand::prelude::*;

use crate::tsp::TSPInstance;
use super::Assignment;
use crate::modulo;

#[derive(Clone, Debug)]
pub struct Solution {
    assignments: Vec<Assignment>,
    unassigned_vertices: Vec<u32>,
    driver_distances: Vec<usize>,
    instance: Rc<TSPInstance>,
    objective_value: usize
}

impl Solution {
    pub fn new(instance: Rc<TSPInstance>) -> Self {
        Solution {
            assignments: Vec::with_capacity(instance.number_of_vertices()),
            unassigned_vertices: (1..instance.number_of_vertices() as u32).collect(),
            driver_distances: vec![0; instance.number_of_drivers()],
            instance,
            objective_value: usize::max_value()
        }
    }

    pub fn new_random(instance: Rc<TSPInstance>) -> Self {
        let mut rng = rand::thread_rng();
        let mut vertices: Vec<u32> = (1..instance.number_of_vertices() as u32).collect();
        let mut solution = Solution::new(instance);
        vertices.shuffle(&mut rng);
        for vertex in vertices.iter() {
            let driver = rand::thread_rng().gen_range(0, solution.instance().number_of_drivers() as u32);
            let last_vertex = solution.get_last_vertex();
            solution.add_assignment(*vertex, driver, solution.instance().get_vertex(*vertex).get_weight(last_vertex));
        }
        let driver = rand::thread_rng().gen_range(0, solution.instance().number_of_drivers() as u32);
        let last_vertex = solution.get_last_vertex();
        solution.add_assignment(0, driver, solution.instance().get_vertex(0).get_weight(last_vertex));
        solution
    }

    pub fn unassigned_vertices(&self) -> &Vec<u32> {
        &self.unassigned_vertices
    }

    pub fn objective_value(&self) -> usize {
        self.objective_value
    }

    pub fn driver_distances(&self) -> &Vec<usize> {
        &self.driver_distances
    }

    pub fn get_driver_distance(&self, idx: usize) -> usize {
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

    pub fn get_distance(&self, idx: usize) -> usize {
        let first = self.get_assignment(idx).vertex();
        let prev = self.get_assignment(modulo(idx as isize - 1, self.instance.number_of_vertices())).vertex();
        self.instance.get_vertex(first).get_weight(prev)
    }
    
    /// Calculates the change in objective value given a
    /// delta: change in distance for driver (old - new)
    pub fn delta_evaluation(&mut self, driver: u32, delta: isize) {
        let new_distance = self.driver_distances[driver as usize] as isize - delta;
        let x = self.objective_value() as isize - delta * (-2 * self.instance().desired_travel_distance() as isize + self.driver_distances[driver as usize] as isize + new_distance as isize);
        self.objective_value = x as usize;
        self.driver_distances[driver as usize] = new_distance as usize;
    }

    pub fn add_assignment(&mut self, vertex: u32, driver: u32, distance: usize) {
        if self.assignments.len() > self.instance.number_of_vertices() {
            panic!("Exceeded maximum number of assignments.");
        }
        self.assignments.push(Assignment::new(vertex, driver));        
        self.driver_distances[driver as usize] += distance;

        if vertex != 0 {
            let idx = self.unassigned_vertices.iter().position(|x| *x == vertex).unwrap();  // Remove vertex out of unassigned
            self.unassigned_vertices.remove(idx);
        }
    }

    pub fn get_last_vertex(&self) -> u32 {
        match self.assignments.last() {
            Some(x) => x.vertex(),
            None => 0
        }
    }

    pub fn get_smallest_driver(&self) -> u32 {  // Get driver with smallest distance
        let (best_driver, _) = self.driver_distances.iter().enumerate().min_by_key(|(_, x)| *x).unwrap();
        best_driver as u32
    }
    
    pub fn calculate_objective_value(&mut self) {
        self.objective_value = self.driver_distances.iter().map(|x| (self.instance.desired_travel_distance() as isize - *x as isize).pow(2) as usize).collect::<Vec<usize>>().iter().sum();
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
            let first_vertex = if i == 0 { 0 } else { self.assignments[i-1].vertex() };
            let second_vertex = self.assignments[i].vertex();
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
        let mut result = String::from("0");
        let min = cmp::min(self.instance.number_of_vertices() - 1, self.number_of_assignments());
        for i in 0..min {
            result.push_str(" ");
            result += &self.assignments[i].vertex().to_string();
        }
        result
    }

    pub fn drivers_to_str(&self) -> String {
        let mut result = String::new();
        let min = cmp::min(self.instance.number_of_vertices(), self.number_of_assignments());
        for i in 0..min {
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
