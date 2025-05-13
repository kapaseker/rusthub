use std::cmp::Ordering;
use std::ops::Add;

struct Key {
    id: u32,
    metadata: Option<String>,
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}


#[derive(Eq, PartialEq)]
struct Citation {
    author: String,
    year: u32,
}
impl PartialOrd for Citation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.author.partial_cmp(&other.author) {
            Some(Ordering::Equal) => self.year.partial_cmp(&other.year),
            author_ord => author_ord,
        }
    }
}

impl PartialEq<u32> for Key {
    fn eq(&self, other: &u32) -> bool {
        self.id == *other
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Add<(i32,i32)> for Point {
    
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self { x: self.x + rhs.0, y: self.y + rhs.1 }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let a = Key { id: 1, metadata: None };
    let b = Key { id: 2, metadata: None };

    println!("{}", a == b);
    println!("{}", a != b);
    println!("{}", a == 1);
    
    let a = "Hello";
    let b = String::from("Hello");
    assert_eq!(a, b);
    
    println!("{}", !5);
    
    println!("{:?}", Point{x:1,y:2} + (2,4));
    
    let value: i64 = 1000;
    println!("as u16: {}", value as u16);
    println!("as i16: {}", value as i16);
    println!("as u8: {}", value as u8);
}