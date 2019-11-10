#![allow(dead_code)]

mod tsp;

pub use tsp::Algorithm;

use tsp::TestRunner;

pub fn test_instance(instance: &str, algorithm: Algorithm) {
    println!("Solve {}.", instance);
    match algorithm {
        Algorithm::Greedy => println!("Do greedy"),
        Algorithm::RandomGreedy => println!("Do random greedy")
    }
} 