fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longestFirst<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn main() {
    println!("Hello, world!");
    println!("{}", longest("hello", "adele"));

    let str1 = String::from("Apple");
    let theLong;
    {
        let str2 = String::from("Google");
        theLong = longestFirst(&str1, &str2);
    }
    println!("{}",theLong);
}
