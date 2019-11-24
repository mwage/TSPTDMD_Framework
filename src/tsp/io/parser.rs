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
            let vec: Vec<isize> = line.split(' ').map(|x| x.parse().unwrap()).collect();
            points.push(Point::new(vec[0], vec[1]));    // Add point to the list
        }

        for i in 0..number_of_vertices {
            for j in 0..number_of_vertices {
                if i == j {
                    continue;
                }
                let first = &points[i];
                let second = &points[j];
                instance.add_edge(i, j, first.calculate_distance(second));  // Add edges between all points using the calculated distance
            }
        }

        instance
    }

    // Create TSP instance from EDGELIST
    fn get_edge_list(mut lines: VecDeque<&str>) -> TSPInstance {
        println!("Get EDGELIST");
        let vec: Vec<usize> = lines.pop_front().unwrap().split(' ').map(|x| x.parse().unwrap()).collect();    // Get the parameters from the second line
        let (number_of_vertices, _, number_of_drivers, desired_travel_distance) = (vec[0], vec[1], vec[2], vec[3]);

        let mut instance = TSPInstance::new(number_of_vertices, number_of_drivers, desired_travel_distance);   // Create TSP instance

        let mut max = 0;
        for line in lines.iter() {
            if line.is_empty() {
                continue;
            }
            let vec: Vec<usize> = line.split(' ').map(|x| x.parse().unwrap()).collect();
            instance.add_edge(vec[0], vec[1], vec[2] as isize);    // Add an edge for each line in the file
            
            if vec[2] > max {
                max = vec[2];
            }
        }

        instance.complete_graph((number_of_vertices * max + 1) as isize);  // Set weight for all unspecified edges

        instance
    }
}