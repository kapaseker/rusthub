use std::io::{Error, IntoInnerError, Read};
use std::{fs, io};
use std::fmt::{Debug, Display, Formatter};
use std::ptr::NonNull;

struct ParseError;

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for ParseError {}

fn main() {
    let v = vec![10, 20, 30];
    println!("{}", v.get(100).unwrap_or(&-1));
    println!("{}", v.get(1).unwrap_or(&-1));

    let op = Option::Some(10);
    let res = op.ok_or(ParseError);

    let res: Result<i32, ParseError> = Ok(10);
    let op = res.ok();
}



fn read_username(path: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    fs::File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
}