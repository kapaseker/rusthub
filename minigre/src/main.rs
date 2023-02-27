use std::env;
use std::fs;
fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("give me arguments at least 2");
        return;
    }

    let query = &args[1];
    let path = &args[2];

    println!("search for {query} in file {path}");

    let contents = fs::read_to_string(path).expect("Can't read current file");

    println!("{contents}");
}
