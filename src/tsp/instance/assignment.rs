#[derive(Debug, Copy, Clone)]
pub struct Assignment {
    vertex: usize,
    driver: usize
}

impl Assignment {
    pub fn new(vertex: usize, driver: usize) -> Self {
        Assignment {
            vertex,
            driver
        }
    }

    pub fn vertex(&self) -> usize {
        self.vertex
    }

    pub fn driver(&self) -> usize {
        self.driver
    }

    pub fn from_assignment(&mut self, other: &Assignment) {
        self.vertex = other.vertex();
        self.driver = other.driver();
    }
    
    pub fn set_vertex(&mut self, vertex: usize) {
        self.vertex = vertex;
    }

    pub fn set_driver(&mut self, driver: usize) {
        self.driver = driver;
    }
}