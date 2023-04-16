use std::sync::mpsc;
use std::thread;
use std::time::Duration;


/**
 * channel中，线程结束，发送才结束，这个不是理解就能接收到的。
 */

fn main() {
    let (sender, reciever) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        sender.send(1).unwrap()
    });

    println!("the value from chanel: {}", reciever.recv().unwrap_or(10));

    let (ms, mr) = mpsc::channel();
    let ms_clone = ms.clone();
    thread::spawn(move || {
        for i in 1..10 {
            ms.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        for i in 11..20 {
            ms_clone.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for r in mr {
        print!("{} ",r);
    }

    println!("");


    let (syncWriter,syncReader) = mpsc::sync_channel(0);

    thread::spawn(move || {
        for i in 11..20 {
            syncWriter.send(i).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });
    

    for i in syncReader {
        print!("{} ",i);
    }

    println!("");
}
