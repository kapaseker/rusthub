use std::collections::HashMap;

fn main() {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert("name".to_string(), "apple".to_string());
    map.insert("age".to_string(), "28".to_string());
    map.insert("alias".to_string(), "pingguo".to_string());

    map.entry("name".to_string()).and_modify(|e| e.push_str(" google"));

    println!("{}",map["name"]);
}
