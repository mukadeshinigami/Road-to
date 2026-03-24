/// Task 3: Point and distance.

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// Computes the Euclidean distance to another point.
    fn distance_to(&self, other: &Point) -> f64 {
        // TODO: Implement: sqrt((dx)^2 + (dy)^2) using `value.sqrt()`.
        let _ = other;
        0.0
    }
}

fn main() {
    let p1 = Point { x: 0.0, y: 0.0 };
    let p2 = Point { x: 3.0, y: 4.0 };

    println!("Distance: {}", p1.distance_to(&p2)); // expected: 5.0
}

