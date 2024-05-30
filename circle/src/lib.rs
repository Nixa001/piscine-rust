#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn distance(&self, val: &Point) -> f64 {
        ((self.x - val.x).powi(2) + (self.y - val.y).powi(2)).sqrt()
    }
}
#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            center: Point { x, y },
            radius,
        }
    }

    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }

    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    pub fn intersect(&self, val2: &Circle) -> bool {
        self.center.distance(&val2.center) < (self.radius + val2.radius)
    }
}


// fn main() {
// 	let circle = Circle::new(500.0, 500.0, 150.0);
// 	let circle1 = Circle {
// 		center: Point { x: 80.0, y: 115.0 },
// 		radius: 30.0,
// 	};
// 	let point_a = Point { x: 1.0, y: 1.0 };
// 	let point_b = Point { x: 0.0, y: 0.0 };
// 	println!("circle = {:?} area = {}", circle, circle.area());
// 	println!("circle = {:?} diameter = {}", circle, circle.diameter());
// 	println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
// 	println!(
// 		"circle and circle1 intersect = {}",
// 		circle.intersect(&circle1)
// 	);

// 	println!(
// 		"distance between {:?} and {:?} is {}",
// 		point_a,
// 		point_b,
// 		point_a.distance(&point_b)
// 	);

// }
// And its output

// $ cargo run
// circle = Circle { center: Point { x: 500.0, y: 500.0 }, radius: 150.0 } area = 70685.83470577035
// circle = Circle { center: Point { x: 500.0, y: 500.0 }, radius: 150.0 } diameter = 300
// circle1 = Circle { center: Point { x: 80.0, y: 115.0 }, radius: 30.0 } diameter = 60
// circle and circle1 intersect = false
// distance between Point { x: 1.0, y: 1.0 } and Point { x: 0.0, y: 0.0 } is 1.4142135623730951
// $