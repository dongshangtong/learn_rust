use std::fmt::Debug;

trait Shape {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
    std::f64::consts::PI * self.radius * self.radius
    }
}

#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
    (self.base * self.height) / 2.0
    }
}

#[derive(Debug)]
struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
    self.side * self.side
    }
}

fn print_area<T>(shape: T) where T: Shape + Debug {
    println!("Shape: {:?}", shape);
    println!("Area: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 2.0 };
    let triangle = Triangle {
        base: 3.0,
        height: 4.0,
    };
    let square = Square { side: 5.0 };

    print_area(circle);
    print_area(triangle);
    print_area(square);
}

// Output:
// Shape: Circle { radius: 2 }
// Area: 12.566370614359172
// Shape: Triangle { base: 3, height: 4 }
// Area: 6
// Shape: Square { side: 5 }
// Area: 25
