use super::Vertex;

#[derive(Debug)]
pub struct TSPInstance {
    number_of_vertices: usize,
    number_of_drivers: usize, 
    desired_travel_distance: usize,
    vertices: Vec<Vertex>
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
            vertices
        }
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
                self.vertices[i].add_edge(j as u32, m);
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
