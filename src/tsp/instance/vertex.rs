use std::collections::HashMap;

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
    }

    pub fn get_weight(&self, idx: u32) -> usize {
        self.edges[&idx]
    }
}