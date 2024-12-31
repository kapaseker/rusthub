use std::collections::HashMap;

pub struct SantaList {
    // 1. Define the records field
    records:HashMap<String, bool>,
}

impl SantaList {
    // 2. Implement the new method
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }
    // 3. Implement the add method
    pub fn add(&mut self, key:&str, value:bool) {
        self.records.insert(key.to_string(), value);
    }
    // 4. Implement the remove method
    pub fn remove(&mut self, key:&str) {
        self.records.remove(&key.to_string());
    }
    // 5. Implement the get method
    pub fn get(&self, key:&str) -> Option<bool> {
        self.records.get(key).cloned()
    }
    // 6. Implement the count method
    pub fn count(&self) -> (usize,usize) {
        let mut counter = (0usize,0usize);
        self.records.iter().for_each(|(_,b)| {
            if *b {
                counter.0 += 1;
            }else {
                counter.1 += 1;
            }
        });
        counter
    }
    // 7. Implement the list_by_behavior method
    pub fn list_by_behavior(&self, good: bool) -> Vec<String> {
        let mut counter:Vec<String> = vec![];
        self.records.iter().for_each(|(a,&b)| {
            if b == good {
                counter.push(a.clone());
            }
        });
        counter
    }
}

pub fn main() {
    let mut santa_list = SantaList::new();

    santa_list.add("Alice", true);
    santa_list.add("Bob", false);
    santa_list.add("Charlie", true);

    if let Some(behavior) = santa_list.get("Alice") {
        println!(
            "Alice is on the {} list.",
            if behavior { "Nice" } else { "Naughty" }
        );
    }

    let (nice, naughty) = santa_list.count();
    println!(
        "Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );

    let nice_list = santa_list.list_by_behavior(true);
    println!("Nice children: {:?}", nice_list);

    let naughty_list = santa_list.list_by_behavior(false);
    println!("Naughty children: {:?}", naughty_list);

    santa_list.remove("Bob");
    let (nice, naughty) = santa_list.count();
    println!(
        "After removing Bob, Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );
}
