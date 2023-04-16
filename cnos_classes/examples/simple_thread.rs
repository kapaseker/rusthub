use std::thread;
use std::time::Duration;

fn main() {
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("hi {} from thread.", i);
            thread::sleep(Duration::from_millis(100));
        }
        return 100;
    });

    for i in 1..5 {
        println!("hi {} from main thread.", i);
        thread::sleep(Duration::from_millis(100));
    }

   let result = handler.join().unwrap_or(0);

   println!("thread result is {}", result);
}
