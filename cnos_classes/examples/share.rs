use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

/// A struct that prints which thread drops it.
#[derive(Debug)]
struct WhereDropped(Vec<i32>);

impl Drop for WhereDropped {
    fn drop(&mut self) {
        println!("Dropped by {:?}", thread::current().id())
    }
}

// fn main() {
//     let v = Arc::new(WhereDropped(vec![10, 20, 30]));
//     let mut handles = Vec::new();
//     for i in 0..5 {
//         let v = Arc::clone(&v);
//         handles.push(thread::spawn(move || {
//             // Sleep for 0-500ms.
//             std::thread::sleep(std::time::Duration::from_millis(500 - i * 100));
//             let thread_id = thread::current().id();
//             println!("{thread_id:?}: {v:?}");
//         }));
//     }
//
//     // Now only the spawned threads will hold clones of `v`.
//     drop(v);
//
//     // When the last spawned thread finishes, it will drop `v`'s contents.
//     handles.into_iter().for_each(|h| h.join().unwrap());
// }

fn main() {
    let a = Arc::new(Mutex::new(45));
    let mut handles = Vec::new();
    for i in 0..1000 {
        let a = a.clone();
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut a = a.lock().unwrap();
                *a += 10;
                drop(a);
                sleep(Duration::from_millis(1));
            }
        });
        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());

    println!("{:?}", *a.lock().unwrap());
}