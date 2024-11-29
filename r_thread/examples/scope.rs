use std::sync::{Arc, Mutex};
use std::thread;

fn main() {

    let nums = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

    thread::scope(|scope| {

        scope.spawn(|| {
            println!("len {}", nums.lock().unwrap().len());
        });

        scope.spawn(|| {
            for i in nums.lock().unwrap().iter() {
                print!("{} ", i);
            }
            println!()
        });

        scope.spawn(|| {
            nums.lock().unwrap().push(9);
        });
    });
}
