enum Shape {
    Circle(f32),
    Rectangle(f32, f32),
    Triangle(f32, f32, f32),
}

impl Shape {
    fn perimeter(&self) -> f32 {
        match &self {
            Self::Circle(radius) => 2.0 * std::f32::consts::PI * radius,
            Self::Rectangle(width, height) => 2.0 * (width + height),
            Self::Triangle(a, b, c) => a + b + c
        }
    }
}

fn main() {
    let circle = Shape::Circle(15.0);
    let rect = Shape::Rectangle(24.0, 12.0);
    let triangle = Shape::Triangle(12.2, 14.0, 32.4);

    println!("Perimeter of circle is: {}", circle.perimeter());
    println!("Perimeter of rectangle is: {}", rect.perimeter());
    println!("Perimeter of triangle is: {}", triangle.perimeter());
}