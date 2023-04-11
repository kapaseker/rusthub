use std::any::type_name;

fn print_type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

/**
 * 默认数组实现了copy traint
 */
fn main() {
    let a = [1, 2, 3, 4];
    let b = a;

    println!("{:?}", a);
    println!("{:?}", b);

    let my_number = 32.90;
    println!("{:?}", print_type_of(&my_number));

    println!("{:?}", print_type_of(&a));
    println!("{:?}", print_type_of(&b));
}
