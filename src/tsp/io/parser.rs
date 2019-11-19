use std::fs;
use std::io::Error;
use std::collections::VecDeque;

use crate::tsp::TSPInstance;
use super::Point;

pub struct InstanceParser {
}

impl InstanceParser {
    pub fn get_instance(name: &str) -> Result<TSPInstance, Error> {
        let lines = fs::read_to_string(format!("instances/{}.txt", name))?; // Read text from file
        let mut lines: VecDeque<&str> = lines.split('\n').collect();    // Split them into lines
        match lines.pop_front().unwrap() {  // Match the first line to get the format
            "EDGELIST" => Ok(InstanceParser::get_edge_list(lines)),
            "COORDS" => Ok(InstanceParser::get_coordinate_format(lines)),
            _ => unimplemented!()
        }
    }

    // Create TSP instance from COORDS
    fn get_coordinate_format(mut lines: VecDeque<&str>) -> TSPInstance {
        println!("Get COORDS");
        let vec: Vec<usize> = lines.pop_front().unwrap().split(' ').map(|x| x.parse().unwrap()).collect();    // Get the parameters from the second line
        let (number_of_vertices, number_of_drivers, desired_travel_distance) = (vec[0], vec[1], vec[2]);
        let mut instance = TSPInstance::new(number_of_vertices, number_of_drivers, desired_travel_distance);   // Create TSP instance
        let mut points: Vec<Point> = Vec::new();
        for line in lines.iter() {
            if line.is_empty() {
                continue;
            }
            let vec: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
            points.push(Point::new(vec[0], vec[1]));    // Add point to the list
        }

        for i in 0..number_of_vertices {
            for j in 0..number_of_vertices {
                if i == j {
                    continue;
                }
                let first = &points[i as usize];
                let second = &points[j as usize];
                instance.add_edge(i as u32, j as u32, first.calculate_distance(second));  // Add edges between all points using the calculated distance
            }
        }

        // println!("{:?}", instance);

        instance
    }

    // Create TSP instance from EDGELIST
    fn get_edge_list(mut lines: VecDeque<&str>) -> TSPInstance {
        println!("Get EDGELIST");
        let vec: Vec<usize> = lines.pop_front().unwrap().split(' ').map(|x| x.parse().unwrap()).collect();    // Get the parameters from the second line
        let (number_of_vertices, _, number_of_drivers, desired_travel_distance) = (vec[0], vec[1], vec[2], vec[3]);

        let mut instance = TSPInstance::new(number_of_vertices, number_of_drivers, desired_travel_distance);   // Create TSP instance
        let mut total_edge_weight = 0;  // Sum of all edge weights, used to complete graph

        for line in lines.iter() {
            if line.is_empty() {
                continue;
            }
            let vec: Vec<usize> = line.split(' ').map(|x| x.parse().unwrap()).collect();
            instance.add_edge(vec[0] as u32, vec[1] as u32, vec[2]);    // Add an edge for each line in the file
            total_edge_weight += vec[2];
        }

        instance.complete_graph(total_edge_weight + 1);  // Set weight for all unspecified edges

        // println!("{:?}", instance);

        instance
    }
}