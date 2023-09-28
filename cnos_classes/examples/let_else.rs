enum Shape {
    Cube,
    Circle,
    Line,
}

fn main() {
    println!("{}", is_circle(Shape::Cube));
    println!("{}", is_circle(Shape::Circle));
    println!("{}", is_circle(Shape::Line));
}

fn is_circle(shape: Shape) -> bool {
    if let Shape::Circle = shape {
        true
    } else {
        false
    }
}