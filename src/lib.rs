#![allow(dead_code)]

mod tsp;

pub use tsp::algorithms::Algorithm;

use tsp::TestRunner;

pub fn test_instance(instance: &str, algorithm: Algorithm) {
    TestRunner::run_instance(&algorithm, instance);
}

pub fn test_all_instances(algorithm: Algorithm) {
    TestRunner::run_all_instances(algorithm);
}