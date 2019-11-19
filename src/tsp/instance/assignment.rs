#[derive(Debug, Clone)]
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
}