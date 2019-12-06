use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Vertex {
    edges: HashMap<usize, i64>
}

impl Vertex {
    pub fn new() -> Self {
        Vertex {
            edges: HashMap::new()
        }
    }

    pub fn add_edge(&mut self, idx: usize, weight: i64) {
        self.edges.entry(idx).or_insert(weight);
    }

    pub fn get_weight(&self, idx: usize) -> i64 {
        self.edges[&idx]
    }
}