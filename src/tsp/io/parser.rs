use std::fs;
use std::io::Error;
use std::collections::VecDeque;

use crate::tsp::TSPInstance;

pub struct InstanceParser {
}

impl InstanceParser {
    pub fn get_instance(name: &str) -> Result<TSPInstance, Error> {
        let lines = fs::read_to_string(format!("instances/{}", name))?;
        let mut lines: VecDeque<&str> = lines.split('\n').collect();
        match lines.pop_front().unwrap() {
            "EDGELIST" => Ok(InstanceParser::get_edge_list(lines)),
            _ => unimplemented!()
        }
    }

    fn get_coordinate_format() {
        // TODO: implement
    }

    fn get_edge_list(mut lines: VecDeque<&str>) -> TSPInstance {
        println!("Get EDGELIST");
        let vec: Vec<u32> = lines.pop_front().unwrap().split(' ').map(|x| x.parse().unwrap()).collect();
        let (number_of_vertices, number_of_edges, number_of_drivers, desired_travel_distance) = (vec[0], vec[1], vec[2], vec[3]);
        let mut instance = TSPInstance::new(number_of_vertices, number_of_edges, number_of_drivers, desired_travel_distance);

        for i in 0..number_of_vertices {
            instance.add_vertex(i);
        }

        for line in lines.iter() {
            if line.is_empty() {
                continue;
            }
            let vec: Vec<usize> = line.split(' ').map(|x| x.parse().unwrap()).collect();
            instance.add_edge(vec[0] as u32, vec[1] as u32, vec[2]);

        }
        instance.complete_graph();
        println!("{:?}", instance);

        instance
    }
}