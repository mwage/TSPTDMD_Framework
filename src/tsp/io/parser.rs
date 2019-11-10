use std::fs;

use crate::tsp::TSPInstance;

pub struct InstanceParser {
}

impl InstanceParser {
    pub fn get_instance(name: &str) -> TSPInstance {
        let lines = fs::read_to_string(format!("instances/{}", name)).expect(&format!("Failed to read instance {}.", name));
        // TODO: distinguish between coordinate and edge list, return matching
        TSPInstance {

        }
    }

    fn get_coordinate_format() {
        // TODO: implement
    }

    fn get_edge_list() {
        // TODO: implement
    }
}