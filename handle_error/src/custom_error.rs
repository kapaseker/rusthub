use std::fmt;

pub fn good() {
    println!("this is custom error mod");
}

pub struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "this is my error")
    }
}

impl fmt::Debug for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}

pub fn produceMyError() -> Result<(), MyError> {
    Err(MyError)
}
