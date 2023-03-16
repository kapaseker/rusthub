use std::{fmt::Display, io::SeekFrom};

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

    fn square(side_len: f32) -> Self {
        return Self {
            width: side_len,
            height: side_len,
        };
    }
}

enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddress {
    fn print(&self) {
        match self {
            IpAddress::V4(_, _, _, _) => {
                println!("this is ip v4.")
            }
            IpAddress::V6(x) => {
                println!("this is ip v6: {}.", x)
            }
        }
    }
}

#[derive(Debug)]
struct Power {
    name: String,
    age: i32,
}

impl Display for Power {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "My name is {}, i am {} years old.", self.name, self.age)
    }
}

fn main() {
    let power = Power {
        name: String::from("Kaka"),
        age: 34,
    };

    println!("{}", power);
    println!("{:?}", power);

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

    let ip_v6 = IpAddress::V6(String::from("::3"));
    ip_v6.print();
}
