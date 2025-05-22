fn main() {
    let center = Point::center();
    let n = 100000;
    let mut c = 0;
    for _ in 0..n {
        if Point::new().in_circle(&center) {
            c += 1; 
        }
    }
    println!("total beans = {n}\nWithin Circle = {c}\n Area (radius is 0.5) = {}", c as f64 / n as f64)
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
        ((x_p1-x_p2).powi(2) + (y_p1-y_p2).powi(2)).sqrt()
    }

    fn in_circle(&self, center: &Point) -> bool {
        self.distance(center) < 0.5
    }
}

