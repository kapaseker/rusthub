use std::io;
use std::io::Write;

fn main() {
    println!("I'm picking a number between 1 and 100...");

    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let mut val = String::new();

    io::stdin().read_line(&mut val)
        .expect("Error getting guess");

    println!("You entered {}", val);
}