use std::f64::consts::PI;

trait CalcArea {
    fn calc_area(&self) -> f64;
}

struct Circle {
    r: f64,
}

struct Rectangle {
    w: f64,
    h: f64,
}

impl CalcArea for Circle {
    fn calc_area(&self) -> f64 {
        PI * self.r * self.r
    }
}

impl CalcArea for Rectangle {
    fn calc_area(&self) -> f64 {
        self.w * self.h
    }
}

fn print_area<T>(shape: T)
where
    T: CalcArea,
{
    println!("shape area is : {}", shape.calc_area());
}

#[test]
fn test_calc_area() {
    let circle = Circle { r: 2.0 };
    let rectangle = Rectangle { w: 4.11, h: 6.0 };
    print_area(circle);
    print_area(rectangle);
}
