use std::ops::Add;

struct Counter {
    max: u32,
    pub num: u32,
}

impl Counter {
    fn create(max: u32) -> Self {
        Self { max, num: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.num < self.max {
            self.num = self.num + 1;
            return Some(self.num);
        } else {
            return None;
        }
    }
}

impl Add for Counter {
    type Output = Counter;

    fn add(self, rhs: Self) -> Self::Output {
        return Counter {
            max: self.max + rhs.max,
            num: 0,
        };
    }
}

fn main() {
    let mut counter = Counter::create(10);

    for i in counter.into_iter() {
        print!("{}", i);
    }

    println!("----sep----");

    let sumCounter = Counter::create(10) + Counter::create(12);

    for i in sumCounter {
        print!("{}", i);
    }

    println!("----sep----");

    let v = vec![1,2,3,4,5];

    for i in v.iter() {
        print!("{}", i);
    }

    println!("----sep----");
}
