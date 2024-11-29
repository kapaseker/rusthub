use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let nums = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

    let a = thread::spawn({
        let nums = nums.clone();
        move || {
            println!("len {}", nums.lock().unwrap().len());
        }
    });

    let b = thread::spawn({
        let nums = nums.clone();
        move || {
            for i in nums.lock().unwrap().iter() {
                print!("{} ", i);
            }
            println!()
        }
    });

    let c = thread::spawn({
        let nums = nums.clone();
        move || {
            nums.lock().unwrap().push(9);
        }
    });

    a.join().unwrap();
    b.join().unwrap();
    c.join().unwrap();
}
