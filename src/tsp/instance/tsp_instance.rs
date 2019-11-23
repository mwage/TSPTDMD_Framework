use crate::rand::Rng;
use super::Vertex;


#[derive(Debug, Clone)]
pub struct TSPInstance {
    number_of_vertices: usize,
    number_of_drivers: usize, 
    desired_travel_distance: usize,
    vertices: Vec<Vertex>,
    invalid_weight: Option<usize>
}

impl TSPInstance {
    pub fn new(number_of_vertices: usize, number_of_drivers: usize, desired_travel_distance: usize) -> Self {
        let mut vertices = Vec::with_capacity(number_of_vertices);
        for _ in 0..number_of_vertices {    // Add all vertices to the instance
            vertices.push(Vertex::new());
        }

        TSPInstance {
            number_of_vertices,
            number_of_drivers,
            desired_travel_distance,
            vertices,
            invalid_weight: None
        }
    }

    pub fn new_random(number_of_vertices: usize, number_of_drivers: usize, desired_travel_distance: usize, max_distance : usize) -> Self {
        let mut instance = TSPInstance::new(number_of_vertices, number_of_drivers, desired_travel_distance);

        for i in 0..number_of_vertices as u32 {
            for j in i..number_of_vertices as u32 {
                instance.add_edge(i, j, rand::thread_rng().gen_range(0, max_distance));
            }
        }

        instance
    }

    pub fn get_vertex(&self, idx: u32) -> &Vertex {
        &self.vertices[idx as usize]
    }

    pub fn number_of_drivers(&self) -> usize {
        self.number_of_drivers
    }

    pub fn number_of_vertices(&self) -> usize {
        self.number_of_vertices
    }
    
    pub fn desired_travel_distance(&self) -> usize {
        self.desired_travel_distance
    }

    pub fn has_only_feasible_edges(&self) -> bool {
        self.invalid_weight == None
    }

    pub fn is_valid(&self, first_vertex: u32, second_vertex: u32) -> bool {
        match self.invalid_weight {
            Some(x) => self.get_vertex(first_vertex).get_weight(second_vertex) != x,
            None => true
        }        
    }

    pub fn add_edge(&mut self, first: u32, second: u32, weight: usize) {
        self.vertices[first as usize].add_edge(second, weight);
        self.vertices[second as usize].add_edge(first, weight);
    }

    pub fn complete_graph(&mut self, m: usize) {
        self.invalid_weight = Some(m);
        for i in 0..self.number_of_vertices {
            for j in 0..self.number_of_vertices {
                if i == j {
                    continue;
                }
                self.vertices[i].add_edge(j as u32, m);
            }
        }
    }
}
