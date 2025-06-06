// use std::sync::{mpsc, Arc, Mutex};
// use std::thread;

use rand::Rng;

// fn main() {
//     let mut rng = rand::thread_rng();
//     let mut vec = Vec::with_capacity(100);
//     for i in 0..100 {
//         vec.push(rng.gen_range(1..=100));
//     }
//
//     let len = vec.len();
//     let mut vecLock = Arc::new(Mutex::new(vec));
//     let mut tasks = vec![];
//
//     for i in 0..len {
//         let lock = vecLock.clone();
//         let hd = thread::spawn(move || {
//             let a = lock.lock().unwrap().pop().unwrap();
//             calculate(a)
//         });
//         tasks.push(hd);
//     }
//
//     let mut sum = 0;
//     for task in tasks {
//         sum += task.join().unwrap();
//     }
//
//     println!("{:?}", sum);
// }
//
// fn calculate(a: i32) -> i32 {
//     a + 199
// }

use std::thread;
use std::time::Duration;

// fn main() {
//    let handle = thread::spawn(|| {
//         for i in 0..10 {
//             println!("Count in thread: {i}!");
//             thread::sleep(Duration::from_millis(5));
//         }
//     });
//
//     for i in 0..5 {
//         println!("Main thread: {i}");
//         thread::sleep(Duration::from_millis(5));
//     }
//
//     handle.join();
// }

fn main() {
    let adder = 4;

    thread::scope(|scope| {
        let handle =  scope.spawn(|| {
            1 + adder
        });
        
        println!("{}", handle.join().unwrap());
    });
    
    println!("{}", adder);
}

// fn foo() {
//     let s = String::from("Hello");
//     thread::scope(|scope| {
//         scope.spawn(|| {
//             dbg!(s.len());
//         });
//     });
// }
//
// fn main() {
//     foo();
// }
