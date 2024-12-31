use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    let x = AtomicUsize::new(0);

    thread::scope(|scope| {
        scope.spawn(|| {
            x.fetch_add(1, Ordering::Relaxed);
        });

        while x.load(Ordering::Relaxed) == 0 {}
        println!("Done");
        println!("{}", x.load(Ordering::Relaxed));
    });

    let y = &AtomicUsize::new(0);

    thread::scope(|scope| {
        scope.spawn(move || {
            y.fetch_add(1, Ordering::Relaxed);
        });
    });

    println!("{}", y.load(Ordering::Relaxed));
}
