use super::Vertex;

#[derive(Debug)]
pub struct TSPInstance {
    pub number_of_vertices: u32,
    pub number_of_drivers: u32, 
    pub desired_travel_distance: u32,
    vertices: Vec<Vertex>
}

impl TSPInstance {
    pub fn new(number_of_vertices: u32, number_of_drivers: u32, desired_travel_distance: u32) -> Self {
        let mut vertices = Vec::with_capacity(number_of_vertices as usize);
        for _ in 0..number_of_vertices {    // Add all vertices to the instance
            vertices.push(Vertex::new());
        }

        TSPInstance {
            number_of_vertices,
            number_of_drivers,
            desired_travel_distance,
            vertices
        }
    }

    pub fn get_vertex(&self, idx: usize) -> &Vertex {
        &self.vertices[idx]
    }

    pub fn add_edge(&mut self, first: u32, second: u32, weight: usize) {
        self.vertices[first as usize].add_edge(second, weight);
        self.vertices[second as usize].add_edge(first, weight);
    }

    pub fn complete_graph(&mut self, m: usize) {
        for i in 0..self.number_of_vertices {
            for j in 0..self.number_of_vertices {
                if i == j {
                    continue;
                }
                self.vertices[i as usize].add_edge(j, m);
            }
        }
    }

    pub fn new_test_instance() -> Self {
        let mut instance = TSPInstance::new(3, 1, 5);
        instance.add_edge(0, 1, 2);
        instance.add_edge(1, 2, 2);
        instance.add_edge(2, 0, 1);
        instance
    }
}
