use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Vertex {
    edges: HashMap<usize, isize>
}

impl Vertex {
    pub fn new() -> Self {
        Vertex {
            edges: HashMap::new()
        }
    }

    pub fn add_edge(&mut self, idx: usize, weight: isize) {
        self.edges.entry(idx).or_insert(weight);
    }

    pub fn get_weight(&self, idx: usize) -> isize {
        self.edges[&idx]
    }
}