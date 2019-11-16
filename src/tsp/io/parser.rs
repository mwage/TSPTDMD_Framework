use std::fs;
use std::io::Error;

use crate::tsp::TSPInstance;

pub struct InstanceParser {
}

impl InstanceParser {
    pub fn get_instance(name: &str) -> Result<TSPInstance, Error> {
        let lines = fs::read_to_string(format!("instances/{}", name))?;
        // TODO: distinguish between coordinate and edge list, return matching
        Ok(TSPInstance {

        })
    }

    fn get_coordinate_format() {
        // TODO: implement
    }

    fn get_edge_list() {
        // TODO: implement
    }
}