use std::rc::Rc;

use crate::rand::Rng;
use crate::tsp::Solution;
use crate::tsp::TSPInstance;
use super::NeighborhoodImpl;
use crate::tsp::io::Logger;

#[derive(Debug)]
pub struct DriverFlip {
    stored_move: Option<DFMove>
}

impl DriverFlip {
    pub fn new() -> Self {
        DriverFlip {
            stored_move: None
        }
    }

    fn stored_move(&self) -> &DFMove {
        match &self.stored_move {
            Some(x) => &x,
            None => panic!("Attempted to set non-initialized neighbor.")
        }        
    }

    fn apply(&mut self, solution: &mut Solution, delta_eval: bool) {
        let (idx, new_driver, delta, distances) = self.stored_move().to_tuple();
        let old_driver = solution.get_assignment(idx).driver();
        let vertex = solution.get_assignment(idx).vertex();
        solution.get_assignment_mut(idx).set_driver(new_driver);

        if delta_eval {
            solution.delta_evaluation(delta, distances);
        }

        // let prev_vertex = solution.get_assignment(modulo_pos(idx as isize - 1, solution.instance().number_of_vertices())).vertex();
        // let distance = solution.instance().get_vertex(prev_vertex).get_weight(vertex);
        // solution.delta_evaluation(old_driver, distance);
        // solution.delta_evaluation(new_driver, -distance);
    }

    fn evaluate_move(&self, solution: &Solution, idx: usize, new_driver: usize) -> DFMove {
        let distance = solution.get_distance(idx);
        let old_driver = solution.get_assignment(idx).driver();
        let old_driver_distance = solution.get_driver_distance(old_driver) - distance;
        let new_driver_distance = solution.get_driver_distance(new_driver);
        let delta = 2 * distance * (new_driver_distance - old_driver_distance);

        let mut distances = solution.driver_distances().clone();
        distances[old_driver] = old_driver_distance;
        distances[new_driver] = new_driver_distance + distance;
        
        DFMove::new(idx, new_driver, delta, distances)
    }
} 

impl NeighborhoodImpl for DriverFlip {
    fn get_random_neighbor(&mut self, solution: &Solution, delta_eval: bool) -> bool {
        let instance = solution.instance();
        if instance.number_of_drivers() == 1 {
            return false;
        }
        let idx = rand::thread_rng().gen_range(0, instance.number_of_vertices());
        let old_driver = solution.get_assignment(idx).driver();
        let mut new_driver = rand::thread_rng().gen_range(0, instance.number_of_drivers() - 1);
        if new_driver >= old_driver {
            new_driver += 1;
        }
        self.stored_move = Some(self.evaluate_move(solution, idx, new_driver));
        true
    }

    fn get_best_improving_neighbor(&mut self, solution: &Solution, delta_eval: bool, logger: &Logger) -> bool {
        self.stored_move = None;
        if solution.instance().number_of_drivers() == 1 {
            return false;
        }
        let number_of_vertices = solution.instance().number_of_vertices();
        let (smallest_driver, _) = solution.driver_distances().iter().enumerate()
            .min_by_key(|(_, dist)| *dist).unwrap();    // Get driver with smallest distance
        for i in 0..number_of_vertices {
            if solution.get_assignment(i).driver() == smallest_driver {
                continue;
            }

            let df_move = self.evaluate_move(solution, i, smallest_driver);

            // If move is not set or delta < delta of stored solution => update stored move
            if let Some(delta) = self.delta() {  
                if df_move.delta() < delta {
                    self.stored_move = Some(df_move);
                }
            } else {
                self.stored_move = Some(df_move);
            }

            if logger.get_elapsed() >= crate::TIME_LIMIT {
                return match &self.stored_move {
                    Some(df_move) => df_move.delta() < 0,
                    None => false
                };
            }
        }

        match &self.stored_move {
            Some(df_move) => df_move.delta() < 0,
            None => false
        }
    }

    fn get_first_improving_neighbor(&mut self, solution: &Solution, delta_eval: bool, logger: &Logger) -> bool {
        self.stored_move = None;
        if solution.instance().number_of_drivers() == 1 {
            return false;
        }
        let number_of_vertices = solution.instance().number_of_vertices();
        let (smallest_driver, _) = solution.driver_distances().iter().enumerate()
            .min_by_key(|(_, dist)| *dist).unwrap();    // Get driver with smallest distance
        for i in 0..number_of_vertices {
            if solution.get_assignment(i).driver() == smallest_driver {
                continue;
            }
            let df_move = self.evaluate_move(solution, i, smallest_driver);

            if df_move.delta() < 0 {
                self.stored_move = Some(df_move);
                return true;
            }
            
            if logger.get_elapsed() >= crate::TIME_LIMIT {
                return false;
            }
        }
        false
    }

    fn set_neighbor(&mut self, solution: &mut Solution, delta_eval: bool) {
        self.apply(solution, delta_eval);
        self.stored_move = None;
    }

    fn delta(&self) -> Option<isize> {
        match &self.stored_move {
            Some(x) => Some(x.delta),
            None => None
        }
    }

    fn to_string(&self) -> String {
        String::from("df")
    }
}

#[derive(Clone, Debug)]
struct DFMove {
    idx: usize,
    new_driver: usize,
    delta: isize,
    distances: Vec<isize>
}

impl DFMove {
    pub fn new(idx: usize, new_driver: usize, delta: isize, distances: Vec<isize>) -> Self {
        DFMove {
            idx,
            new_driver,
            delta,
            distances
        }
    }

    pub fn delta(&self) -> isize {
        self.delta
    }

    pub fn to_tuple(&self) -> (usize, usize, isize, Vec<isize>) {
        (self.idx, self.new_driver, self.delta, self.distances.clone())
    }
}

#[test]
fn test_driver_flip() {
    let instance = TSPInstance::new(1, 2, 10);
    let mut solution = Solution::new(Rc::new(instance));
    solution.add_assignment(0, 0, 10);
    assert_eq!(solution.assignments().len(), 1);
    assert_eq!(solution.get_assignment(0).driver(), 0);
    let mut driver_flip = DriverFlip::new();
    driver_flip.stored_move = Some(DFMove::new(0, 1, 0, Vec::new()));
    driver_flip.apply(&mut solution, false);
    assert_eq!(solution.get_assignment(0).driver(), 1);
}

#[test]
fn test_delta() {
    for _ in 0..100 {
        let instance = TSPInstance::new_random(10, 3, 100, 50);
        let mut solution = Solution::new_random(Rc::new(instance));
        solution.calculate_objective_value();
        let idx = rand::thread_rng().gen_range(0, 10);
        let old_driver = solution.get_assignment(idx).driver();
        let new_driver = (old_driver + 1) % 3;
        let mut driver_flip = DriverFlip::new();
        driver_flip.stored_move = Some(driver_flip.evaluate_move(&solution, idx, new_driver));
        driver_flip.apply(&mut solution, true);
        let new_val = solution.objective_value();
        solution.calculate_objective_value_from_scratch();
        assert_eq!(new_val, solution.objective_value());
    }
}