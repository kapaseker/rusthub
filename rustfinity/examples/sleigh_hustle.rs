use std::thread;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
pub trait SleighTask {
    fn describe(&self) -> String;
}
pub struct SantaSleighQueue {
    records: Mutex<VecDeque<Box<dyn SleighTask + Send>>>
}
impl SantaSleighQueue {
    // 2. Define the `new` constructor
    pub fn new() -> Self {
        Self {
            records: Mutex::new(VecDeque::new())
        }
    }
    // 3. Define the `enqueue` method
    // Since Mutex allows interior mutability, you don't need to get a mutable reference to the self object, you can just use &self.
    pub fn enqueue(&self, task: Box<dyn SleighTask  + Send>) {
        let mut records = self.records.lock().unwrap();
        records.push_back(task);
        ()
    }
    // 4. Define the `get_task` method
    pub fn get_task(&self) -> Option<Box<dyn SleighTask  + Send>> {
        let mut records = self.records.lock().unwrap();
        records.pop_front()
    }
}
pub struct ElfTask {
    // 5. Define the fields
    name: String,
    urgency: u32
}
impl ElfTask {
    // 6. Define the `new` constructor
    pub fn new(name: &str, urgency: u32) -> Self {
        Self {
            name: name.into(),
            urgency
        }
    }
}
impl SleighTask for ElfTask {
    fn describe(&self) -> String {
        format!("Elf task: {} (urgency {})", self.name, self.urgency)
    }
}
pub struct ReindeerTask {
    // 7. Define the
    name: String,
    weight: u32
}
impl ReindeerTask {
    // 8. Define the `new` constructor
    pub fn new(name: &str, weight: u32) -> Self {
        Self {
            name: name.into(),
            weight
        }
    }
}
impl SleighTask for ReindeerTask {
    fn describe(&self) -> String {
        format!("Reindeer task: {} ({} kg)", self.name, self.weight)
    }
}
pub fn main() {
    let queue = Arc::new(SantaSleighQueue::new());
    let producer_queue = Arc::clone(&queue);
    let producer = thread::spawn(move || {
        producer_queue.enqueue(Box::new(ReindeerTask::new("Deliver Toys", 100)));
        producer_queue.enqueue(Box::new(ElfTask::new("Wrap Gifts", 3)));
        producer_queue.enqueue(Box::new(ReindeerTask::new("Collect Reindeer Feed", 50)));
        producer_queue.enqueue(Box::new(ElfTask::new("Decorate Tree", 7)));
    });
    thread::sleep(std::time::Duration::from_millis(10));
    let consumer_queue = Arc::clone(&queue);
    let consumer = thread::spawn(move || loop {
        if let Some(task) = consumer_queue.get_task() {
            println!("{}", task.describe());
        } else {
            break;
        }
    });
    producer.join().unwrap();
    consumer.join().unwrap();
}