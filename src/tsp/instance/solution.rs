use std::rc::Rc;

use crate::tsp::TSPInstance;

#[derive(Clone)]
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

    pub fn assignments(&self) -> &Vec<Assignment> {
        &self.assignments
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

    pub fn number_of_assignments(&self) -> usize {
        self.assignments.len()
    }

    pub fn is_complete(&self) -> bool {
        self.assignments.len() == self.instance.number_of_vertices() - 1
    }

    pub fn print(&self) {
        println!("{:?}", self.assignments.last());
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

    pub fn get_smallest_driver(&self) -> u32 {
        let mut min_distance = usize::max_value();        
        let mut best_driver = u32::max_value();
        for i in 0..self.instance.number_of_drivers() {
            let distance = self.get_driver_distance(i);
            if distance < min_distance {
                min_distance = distance;
                best_driver = i as u32;
            }
        }

        best_driver
    }

    pub fn vertices_to_str(&self) -> String {
        String::from("1,2,3")
    }

    pub fn drivers_to_str(&self) -> String {
        String::from("1,2,3")
    }

    pub fn calculate_objective_value(&mut self) {
        self.objective_value = self.driver_distances.iter().map(|x| (self.instance.desired_travel_distance() - *x).pow(2)).collect::<Vec<usize>>().iter().sum();
    }
}

#[test]
fn test_obj_function() {
    let instance = TSPInstance::new_test_instance();
    let mut solution = Solution::new(Rc::new(instance));
    assert_eq!(solution.objective_value, 25);
    solution.add_assignment(1, 0, 2);
    assert_eq!(solution.objective_value, 9);
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub vertex: u32,
    driver: u32
}

impl Assignment {
    pub fn new(vertex: u32, driver: u32) -> Self {
        Assignment {
            vertex,
            driver
        }
    }
}