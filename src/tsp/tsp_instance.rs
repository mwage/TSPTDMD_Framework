use std::collections::HashMap;

#[derive(Debug)]
pub struct TSPInstance {
    number_of_vertices: u32,
    number_of_edges: u32, 
    number_of_drivers: u32, 
    desired_travel_distance: u32,
    vertices: HashMap<u32, Vertex>,
    m: usize
}

impl TSPInstance {
    pub fn new(number_of_vertices: u32, number_of_edges: u32, number_of_drivers: u32, desired_travel_distance: u32) -> Self {
        TSPInstance {
            number_of_vertices,
            number_of_edges,
            number_of_drivers,
            desired_travel_distance,
            vertices: HashMap::new(),
            m: 1000000
        }
    }

    pub fn add_vertex(&mut self, idx: u32) {
        self.vertices.insert(idx, Vertex::new());
    }

    pub fn add_edge(&mut self, first: u32, second: u32, weight: usize) {
        self.vertices.get_mut(&first).unwrap().add_edge(second, weight);
        self.vertices.get_mut(&second).unwrap().add_edge(first, weight);
    }

    pub fn complete_graph(&mut self) {
        for vertex in self.vertices.values_mut() {
            for i in 0..self.number_of_vertices {
                vertex.add_edge(i, self.m);
            }
        }

        for i in 0..self.number_of_vertices {

        }
    }
}

#[derive(Debug)]
pub struct Vertex {
    edges: HashMap<u32, usize>
}

impl Vertex {
    pub fn new() -> Self {
        Vertex {
            edges: HashMap::new()
        }
    }

    pub fn add_edge(&mut self, idx: u32, weight: usize) {
        self.edges.entry(idx).or_insert(weight);
        // self.edges.insert(idx, weight);
    }
}