// fn main() {
//     let mut max_value = 2;
//     let mut clamp = |v| {
//         if v > max_value {
//             max_value *= 10;
//             max_value
//         } else {
//             v
//         }
//     };
//
//     dbg!(clamp(1));
//     dbg!(clamp(3));
//
//     println!("{}", max_value);
// }

// fn main() {
//     let mut max_value = 2;
//     let mut clamp = |v| {
//         if v > max_value {
//             max_value *= 10;
//             max_value
//         } else {
//             v
//         }
//     };
//
//     println!("{}", max_value);
//
//     dbg!(clamp(1));
//     dbg!(clamp(3));
//
//     println!("{}", max_value);
// }

fn print_type<T>(_: &T) { 
    println!("{:?}", std::any::type_name::<T>());
}

fn main() {
    let max_value = 2;
    let clamp = move |v| {
        if v > max_value {
            max_value
        } else {
            v
        }
    };

    println!("{}", std::any::type_name::<Option<String>>());
    
    print_type(&clamp);
    
    dbg!(clamp(1));
    dbg!(clamp(3));
    
}

// fn main() {
//     let str = String::from("Hello");
//
//     let greet = move |v| {
//         println!("{} {}",str, v);
//     };
//
//     greet("World");
//     greet("Rust");
//
//     // println!("{}", str);
// }

//
// fn apply_and_log(
//     func: impl FnOnce(&'static str) -> String,
//     func_name: &'static str,
//     input: &'static str,
// ) {
//     println!("Calling {func_name}({input}): {}", func(input))
// }
//
// fn main() {
//     let suffix = "-itis";
//     let add_suffix = |x| format!("{x}{suffix}");
//     apply_and_log(&add_suffix, "add_suffix", "senior");
//     apply_and_log(&add_suffix, "add_suffix", "appenix");
//
//     let mut v = Vec::new();
//     let mut accumulate = |x| {
//         v.push(x);
//         v.join("/")
//     };
//     apply_and_log(&mut accumulate, "accumulate", "red");
//     apply_and_log(&mut accumulate, "accumulate", "green");
//     apply_and_log(&mut accumulate, "accumulate", "blue");
//
//     let take_and_reverse = |prefix| {
//         let mut acc = String::from(prefix);
//         acc.push_str(&v.into_iter().rev().collect::<Vec<_>>().join("/"));
//         acc
//     };
//     apply_and_log(take_and_reverse, "take_and_reverse", "reversed: ");
// }


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];
//
//     let mut sort_operations = vec![];
//     let value = String::from("closure called");
//
//     list.sort_by_key(|r| {
//         sort_operations.push(value);
//         r.width
//     });
//     println!("{list:#?}");
// }
