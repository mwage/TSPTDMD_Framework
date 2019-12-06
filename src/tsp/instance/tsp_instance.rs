use crate::rand::Rng;
use super::Vertex;


#[derive(Debug, Clone)]
pub struct TSPInstance {
    number_of_vertices: usize,
    number_of_drivers: usize, 
    desired_travel_distance: i64,
    vertices: Vec<Vertex>,
    invalid_weight: Option<i64>
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
            desired_travel_distance: desired_travel_distance as i64,
            vertices,
            invalid_weight: None
        }
    }

    pub fn new_random(number_of_vertices: usize, number_of_drivers: usize, desired_travel_distance: usize, max_distance : i64) -> Self {
        let mut instance = TSPInstance::new(number_of_vertices, number_of_drivers, desired_travel_distance);

        for i in 0..number_of_vertices {
            for j in i..number_of_vertices {
                instance.add_edge(i, j, rand::thread_rng().gen_range(0, max_distance));
            }
        }

        instance
    }

    pub fn get_vertex(&self, idx: usize) -> &Vertex {
        &self.vertices[idx]
    }

    pub fn number_of_drivers(&self) -> usize {
        self.number_of_drivers
    }

    pub fn number_of_vertices(&self) -> usize {
        self.number_of_vertices
    }
    
    pub fn desired_travel_distance(&self) -> i64 {
        self.desired_travel_distance
    }

    pub fn has_only_feasible_edges(&self) -> bool {
        self.invalid_weight == None
    }

    pub fn is_valid(&self, first_vertex: usize, second_vertex: usize) -> bool {
        match self.invalid_weight {
            Some(x) => self.get_vertex(first_vertex).get_weight(second_vertex) != x,
            None => true
        }        
    }

    pub fn add_edge(&mut self, first: usize, second: usize, weight: i64) {
        self.vertices[first].add_edge(second, weight);
        self.vertices[second].add_edge(first, weight);
    }

    pub fn complete_graph(&mut self, m: i64) {
        self.invalid_weight = Some(m);
        for i in 0..self.number_of_vertices {
            for j in 0..self.number_of_vertices {
                if i == j {
                    continue;
                }
                self.vertices[i].add_edge(j, m);
            }
        }
    }
}
