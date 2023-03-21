use std::env;
use std::fs::File;

mod custom_error;
fn main() {
    let file_result = File::open("./open.txt");

    match file_result {
        Ok(file) => println!("open success"),
        Err(error) => println!("error on open:{:?}", error),
    };

    let home_dir = env::home_dir();
    if let Some(path) = home_dir {
        println!("{}", path.display());
    } else {
        println!("Empty Home Directory");
    }

    custom_error::good();

    match custom_error::produceMyError() {
        Ok(_) => print!("OK"),
        Err(er) => println!("{}", er),
    };
}
