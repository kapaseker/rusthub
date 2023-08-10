use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("foo", 23);
    map.insert("bar", 32);

    let value_of_foo = map.get("foo");
    println!("{}", value_of_foo.copied().unwrap_or(0));

    let value_of_null = map.get("null");
    if let None = value_of_null {
        println!("null is empty");
    }
}