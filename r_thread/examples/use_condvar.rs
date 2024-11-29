use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let condition = Condvar::new();
    let vec = Mutex::new(vec![]);

    thread::scope(|s| {
        s.spawn(|| loop {

            let mut i_lock = vec.lock().unwrap();

            let i = loop {
                if let Some(i) = i_lock.pop() {
                    break i;
                } else {
                    i_lock = condition.wait(i_lock).expect("wait failed");
                }
            };

            println!("{}", i);
        });

        for i in 0..20 {
            vec.lock().unwrap().push(i);
            condition.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
