use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        (PI * self.radius).powi(2)
    }
    fn perimeter(&self) -> f64 {
        2.0 * (PI * self.radius)
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.height * self.width
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.height + self.width)
    }
}

fn main() {
    let circle: Circle = Circle { radius: 5.0 };
    let rectangle: Rectangle = Rectangle { width: 4.0, height: 7.0 };

    println!("Circle Perimeter: {:.2}, Area: {:.2}", circle.perimeter(), circle.area());
    println!("Rectangle Perimeter: {:.2}, Area: {:.2}", rectangle.perimeter(), rectangle.area());
}
