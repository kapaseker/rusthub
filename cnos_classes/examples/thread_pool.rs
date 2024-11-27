use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

use rand::Rng;
use rayon::ThreadPoolBuilder;

fn main() {
    let pool = ThreadPoolBuilder::new().num_threads(4).build().unwrap();

    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(100);
    for i in 0..100 {
        vec.push(rng.gen_range(1..=100));
    }

    let len = vec.len();
    let mut vecLock = Arc::new(Mutex::new(vec));

    let (tx, rx) = channel();

    for i in 0..len {
        let lock = vecLock.clone();
        let tx = tx.clone();
        pool.spawn(move || {
            let a = lock.lock().unwrap().pop().unwrap();
            tx.send(calculate(a)).expect("can't send data!");
        });
    }

    drop(tx);

    let sum = rx.iter().sum::<i32>();

    println!("{:?}", sum);
}

fn calculate(a: i32) -> i32 {
    a + 199
}
