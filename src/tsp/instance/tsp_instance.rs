use std::collections::HashMap;

use super::Vertex;

#[derive(Debug)]
pub struct TSPInstance {
    number_of_vertices: u32,
    number_of_drivers: u32, 
    desired_travel_distance: u32,
    vertices: HashMap<u32, Vertex>
}

impl TSPInstance {
    pub fn new(number_of_vertices: u32, number_of_drivers: u32, desired_travel_distance: u32) -> Self {
        TSPInstance {
            number_of_vertices,
            number_of_drivers,
            desired_travel_distance,
            vertices: HashMap::new()
        }
    }

    pub fn add_vertex(&mut self, idx: u32) {
        self.vertices.insert(idx, Vertex::new());
    }

    pub fn add_edge(&mut self, first: u32, second: u32, weight: usize) {
        self.vertices.get_mut(&first).unwrap().add_edge(second, weight);
        self.vertices.get_mut(&second).unwrap().add_edge(first, weight);
    }

    pub fn complete_graph(&mut self, m: usize) {
        for i in 0..self.number_of_vertices {
            for j in 0..self.number_of_vertices {
                if i == j {
                    continue;
                }
                self.vertices.get_mut(&i).unwrap().add_edge(j, m);
            }
        }
    }
}
