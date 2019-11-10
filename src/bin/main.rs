extern crate tsp;

use tsp::Algorithm;

fn main() {
    tsp::test_instance("test", Algorithm::Greedy);
}
