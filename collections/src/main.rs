fn main() {
    let mut arr = vec![1, 2, 3, 4, 5];
    let first = &mut arr[0];
    *first = 100;
    println!("first of arr is {}", &arr[0]);

    for x in &arr {
        print!("{},", x)
    }

    println!("");

    print!("bytes :");

    let s = String::from("नमस्ते");
    for x in s.bytes() {
        print!("{}, ", x)
    }
    println!("");

    print!("chars :");
    for x in s.chars() {
        print!("{}, ", x)
    }
}
