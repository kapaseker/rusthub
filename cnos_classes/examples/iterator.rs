use std::collections::HashMap;

fn main() {
    let v_one = vec![1, 2, 3, 4];
    println!("{:?}", v_one);

    let v_two: Vec<i32> = v_one.iter().map(|x| x * 2).collect();
    println!("{:?}", v_one);
    println!("{:?}", v_two);

    let solar_distance = HashMap::from([
        ("Mercury".to_string(), 0.4),
        ("Venus".to_string(), 0.7),
        ("Earth".to_string(), 1.0),
        ("Mars".to_string(), 1.5),
    ]);

    iter_map(&solar_distance);
}

fn iter_map(solars: &HashMap<String, f64>) {
    solars.iter().for_each(|(key, value)| {
        println!("{}", key)
    });

    if let Some(v) = solars.get("Venus") {
        println!("Venus has: {}", v);
    }
}