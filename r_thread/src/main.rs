use std::thread;

fn main() {
    let a = thread::spawn(print_thread_id);
    let b = thread::spawn(print_thread_id);

    println!("Hello, Thread!");

    a.join().unwrap();
    b.join().unwrap();

    let numbers = Vec::new();

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap_or(0);

    println!("average: {average}");
}


fn print_thread_id() {
    let id = thread::current().id();
    println!("thread id: {:?}", id);
}
