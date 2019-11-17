#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self{
        Point {
            x,
            y
        }
    }

    pub fn calculate_distance(&self, point: &Point) -> usize {  // Calculates the cartesian distance between two points
        let a = ((self.x - point.x) as f64).powf(2f64);
        let b = ((self.y - point.y) as f64).powf(2f64);
        let dist = (a + b).sqrt().round() as usize;
        dist
    }
}

#[test]
fn test_distance() {
    let first = Point::new(-2, 5);
    let second = Point::new(7, 9);
    let third = Point::new(302, -721);
    assert_eq!(first.calculate_distance(&second), 10);
    assert_eq!(first.calculate_distance(&third), 787);
}