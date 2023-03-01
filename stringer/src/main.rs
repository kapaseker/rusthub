use std::{fmt::format, mem::size_of_val};

struct Pt {
    x: u8,
    y: u8,
}

fn main() {
    let mut str = String::from("Hello, Rust");

    dbg!(size_of_val(&str));

    let replaced = str.replacen("l", "L", 2);

    dbg!(replaced);

    str.replace_range(..8, "G");

    dbg!(&str);

    dbg!(size_of_val(str.as_str()));

    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{} {}!", s1, s2);
    println!("{}", s);
    println!("{}", s1);

    let arr: [String; 8] = core::array::from_fn(|i| format!("it is {}", i));
    dbg!(&arr);
    // 注意这里，迭代器在使用迭代的时候，会移动该引用
    for ele in &arr {}

    for ele in &arr {}

    let pt = Pt { x: 1, y: 2 };
    let Pt { x: m, y: n } = pt;
    println!("{},{}", m, n);

    let Pt { x, y } = pt;
    println!("{},{}", x, y);

    let a = 14;
    match a {
        num @ 1..=10 => {
            println!("num is {}", num);
        },
        b => println!("num is {}", b),
    }

}
