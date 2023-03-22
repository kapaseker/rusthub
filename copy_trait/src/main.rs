#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    // line:Vec<i32>, // The behavior of Copy is not overloadable; it is always a simple bit-wise copy.
}

impl Copy for Point {}

impl Clone for Point {
    fn clone(&self) -> Point {
        return Point {
            x: self.x,
            y: self.y,
            // line:vec![1,2,3],
        };
    }
}

fn main() {
    let a = Point { x: 234, y: 432 };
    let b = a;
    println!("a is {:?}, b is {:?}, you can use a and b at same time.", a, b);
}
