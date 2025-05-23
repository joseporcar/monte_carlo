fn main() {
    let center = Point::center();
    let total_beans = 100000;
    let mut within_circle = 0;
    for _ in 0..total_beans {
        if Point::new().in_circle(&center) {
            within_circle += 1;
        }
    }
    println!(
        "total beans = {total_beans}\nWithin Circle = {within_circle}\nArea (radius is 0.5) = {}",
        within_circle as f64 / total_beans as f64
    )
}

#[derive(Debug)]
struct Point(f64, f64);
impl Point {
    fn new() -> Point {
        Point(rand::random::<f64>(), rand::random::<f64>())
    }

    fn center() -> Point {
        Point(0.5, 0.5)
    }

    fn distance(&self, other: &Point) -> f64 {
        let Point(x_p1, y_p1) = self;
        let Point(x_p2, y_p2) = other;
        ((x_p1 - x_p2).powi(2) + (y_p1 - y_p2).powi(2)).sqrt()
    }

    fn in_circle(&self, center: &Point) -> bool {
        self.distance(center) < 0.5
    }
}
