use std::f64::consts::PI;
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64)
}
impl Shape {
    fn perimeter(&self) -> f64 {
        match *self {
            Shape::Circle(r) => {
                2.0 * r * PI
            },
            Shape::Rectangle(h,w ) => 2.0 * w + 2.0 * h,
            Shape::Triangle(a, b, c) => a+b+c
        }
    }
    fn area(&self) -> f64 {
        match *self {
            Shape::Circle(r) => {
                PI*r*r
            },
            Shape::Rectangle(h, w) => h*w,
            Shape::Triangle(a,b,c) => {
                a * (((c*c-a*a-b*b) / (2.0 * a * b)).acos().sin() * b)*0.5
            }
        }
    }
}
fn main() {
    let ret:Shape = Shape::Circle(5.0);
    println!("perimeter of a\t{:?}\t: {:.4}", ret, ret.perimeter());
    println!("Area of a     \t{:?}\t: {:.4}", ret, ret.area());
    let ret:Shape = Shape::Rectangle(5.0, 10.0);
    println!("perimeter of a\t{:?}\t : {:.4}", ret, ret.perimeter());
    println!("Area of a     \t{:?}\t: {:.4}", ret, ret.area());
    let ret:Shape = Shape::Triangle(3.0, 4.0, 5.0);
    println!("perimeter of a\t{:?}\t: {:.4}", ret, ret.perimeter());
    println!("Area of a     \t{:?}\t: {:.4}", ret, ret.area());
}
