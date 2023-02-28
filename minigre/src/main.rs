use std::{env, process};
use std::error::Error;

use minigre::Config;
use minigre::run;

fn main() {

    let config = Config::build(env::args()).expect("problem with arguments");

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
