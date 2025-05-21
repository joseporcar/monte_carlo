fn main() {
    let center = Point::center();
    let n = 100000;
    let mut c = 0;
    for _ in 0..n {
        if Point::new().in_circle(&center) {
            c += 1; 
        }
    }
    println!("total beans = {n}\nWithin Circle = {c}\n Area = {}", c as f64 / n as f64)
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

    fn in_circle(&self, center: &Point) -> bool {
        let Point(x, y) = self;
        let Point(xc, yc) = center;
        ((x-xc).powi(2) + (y-yc).powi(2)).sqrt() < 0.5
    }
}

