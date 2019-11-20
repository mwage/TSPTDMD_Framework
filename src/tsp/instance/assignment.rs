#[derive(Debug, Copy, Clone)]
pub struct Assignment {
    vertex: u32,
    driver: u32
}

impl Assignment {
    pub fn new(vertex: u32, driver: u32) -> Self {
        Assignment {
            vertex,
            driver
        }
    }

    pub fn vertex(&self) -> u32 {
        self.vertex
    }

    pub fn driver(&self) -> u32 {
        self.driver
    }

    pub fn from_assignment(&mut self, other: &Assignment) {
        self.vertex = other.vertex();
        self.driver = other.driver();
    }
    
    pub fn set_vertex(&mut self, vertex: u32) {
        self.vertex = vertex;
    }

    pub fn set_driver(&mut self, driver: u32) {
        self.driver = driver;
    }
}