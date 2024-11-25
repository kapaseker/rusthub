use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(rng.gen_range(1..=100));
    }
    
    let len = vec.len();
    let mut vecLock = Arc::new(Mutex::new(vec));
    let mut tasks = vec![];

    for i in 0..len {
        let lock = vecLock.clone();
        let hd = thread::spawn(move || {
            let a = lock.lock().unwrap().pop().unwrap();
            calculate(a)
        });
        tasks.push(hd);
    }
    
    let mut sum = 0;
    for task in tasks {
        sum += task.join().unwrap();
    }

    println!("{:?}", sum);
}

fn calculate(a: i32) -> i32 {
    a + 199
}
