struct User {
    id: u64,
    name: String,
    age: u32,
    email: String,
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> f32 {
        return (self.width + self.height) * 2.0;
    }

    fn square(sideLen: f32) -> Self {
        return Self {
            width: sideLen,
            height: sideLen,
        };
    }
}

fn main() {
    let mut bob = User {
        id: 232,
        name: String::from("good"),
        age: 24,
        email: String::from("xpg@163.com"),
    };

    println!("{}", bob.id);
    bob.id += 1;
    println!("{}", bob.id);

    let linda = User {
        id: 1123,
        age: 34,
        ..bob
    };

    println!("{}", linda.age);
    println!("{}", linda.email);

    let square = Rectangle::square(2.6);
    println!("{}", square.area());
    println!("{}", square.perimeter());
}
