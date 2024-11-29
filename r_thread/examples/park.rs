use std::sync::Mutex;
use std::thread;
use std::time::Duration;

fn main() {
    let vec = Mutex::new(vec![]);

    thread::scope(|s| {
        let a = s.spawn(|| loop {
            let mut i = vec.lock().unwrap().pop();
            if let Some(i) = i {
                println!("{}", i);
            } else {
                thread::park();
            }
        });

        for i in 0..20 {
            vec.lock().unwrap().push(i);
            a.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
