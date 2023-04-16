use std::thread::{self, spawn};
use std::time::Duration;

fn main() {
    let mut a = 0;
    let a_hanlder = spawn(move || {
        for i in 1..100 {
            *(&mut a) = &a + 1;
        }
    });

    let b_hanlder = spawn(move || {
        for i in 1..100 {
            a = a + 1
        }
    });

    a_hanlder.join();
    b_hanlder.join();

    println!(" a is {}", a);
}
