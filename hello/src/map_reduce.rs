use std::{char, thread};
use std::sync::mpsc::channel;
use rand::Rng;
use threadpool::ThreadPool;

fn produce(row: usize, col: usize, chars: &Vec<char>) -> u32 {
    let (tx, rx) = channel();
    let a = chars.chunks(col).map(|s| s.into_iter().map(|c| *c).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    a.into_iter().for_each(|col| {
        let tx = tx.clone();
        thread::spawn(move || {
            let sum: u32 = col.iter().map(|c| (*c as u8 - '0' as u8) as u32).sum();
            tx.send(sum).expect("crashed");
        });
    });

    let job = thread::spawn(move || -> u32 {
        let mut sum = 0;
        (0..row).for_each(|_| {
            sum += rx.recv().unwrap();
        });
        sum
    });

    job.join().unwrap()
}

fn produce_pool(row: usize, col: usize, chars: &Vec<char>) -> u32 {
    let (tx, rx) = channel::<u32>();
    let a = chars.chunks(col).map(|s| s.into_iter().map(|c| *c).collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let pool = ThreadPool::new(16);

    a.into_iter().for_each(|col| {
        let tx = tx.clone();
        pool.execute(move || {
            let sum: u32 = col.iter().map(|c| (*c as u8 - '0' as u8) as u32).sum();
            tx.send(sum).expect("crashed");
        })
    });

    let job = thread::spawn(move || -> u32 {
        let mut sum = 0;
        (0..row).for_each(|_| {
            sum += rx.recv().unwrap();
        });
        sum
    });

    job.join().unwrap()
}

#[cfg(test)]
mod test {
    use std::time::SystemTime;
    use rand::Rng;
    use crate::map_reduce::{produce, produce_pool};

    const ROW: usize = 10000;
    const COL: usize = 20000;

    #[test]
    fn case_simple() {
        let mut rand = rand::thread_rng();

        let mut chars= Vec::with_capacity(ROW * COL);
        (0..(ROW * COL)).for_each(|i| {
            chars.push(char::from(rand.gen_range(0u8..=9u8) + '0' as u8));
        });

        let start = SystemTime::now();
        let my_sum: u32 = chars.iter().map(|c| (*c as u8 - '0' as u8) as u32).sum();
        let end = SystemTime::now();
        let gap = end.duration_since(start).unwrap().as_nanos();
        println!("iter_sum time stamp: {:?}", gap);
    }

    #[test]
    fn case_1() {
        let mut rand = rand::thread_rng();


        let mut chars= Vec::with_capacity(ROW * COL);
        (0..(ROW * COL)).for_each(|i| {
            chars.push(char::from(rand.gen_range(0u8..=9u8) + '0' as u8));
        });

        // let my_sum: u32 = chars.iter().map(|c| (*c as u8 - '0' as u8) as u32).sum();

        let start = SystemTime::now();
        let produce_sum = produce(ROW, COL, &chars);
        let end = SystemTime::now();
        let gap = end.duration_since(start).unwrap().as_nanos();
        println!("produce time stamp: {:?}", gap);
        // assert_eq!(my_sum, produce_sum);
    }

    #[test]
    fn case_pool() {
        let mut rand = rand::thread_rng();

        let mut chars= Vec::with_capacity(ROW * COL);
        (0..(ROW * COL)).for_each(|i| {
            chars.push(char::from(rand.gen_range(0u8..=9u8) + '0' as u8));
        });

        // let my_sum: u32 = chars.iter().map(|c| (*c as u8 - '0' as u8) as u32).sum();

        let start = SystemTime::now();
        let produce_sum = produce_pool(ROW, COL, &chars);
        let end = SystemTime::now();
        let gap = end.duration_since(start).unwrap().as_nanos();
        println!("produce_pool time stamp: {:?}", gap);
        // assert_eq!(my_sum, produce_sum);
    }
}