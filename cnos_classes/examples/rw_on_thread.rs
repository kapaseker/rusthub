use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    // 创建线程安全的读写锁
    let data = Arc::new(RwLock::new(0));
    let write_seq = Arc::new(RwLock::new(vec![]));

    // 存储写线程句柄
    let mut writers = vec![];

    // 启动5个写线程
    for i in 0..10 {
        let data_clone = Arc::clone(&data);
        let write_seq_clone = Arc::clone(&write_seq);
        writers.push(thread::spawn(move || {
            for j in 0..1000 {
                // 获取写锁修改数据
                let mut num = data_clone.write().unwrap();
                *num += 1;
                write_seq_clone.write().unwrap().push(i);
                // 模拟写入耗时
                thread::sleep(Duration::from_micros(1));
            }
        }));
    }

    // 等待所有线程结束
    for handle in writers {
        handle.join().unwrap();
    }

    let result = *(data.read().unwrap());
    assert_eq!(result, 10000);
    println!("读取结果是：{}", result);
    println!("写入的线程顺序是：{:?}", *(write_seq.read().unwrap()));
}
