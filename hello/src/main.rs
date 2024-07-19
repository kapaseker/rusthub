use crate::gigasecond::after;
use crate::reverse_string::reverse;

mod block;
mod gigasecond;
mod reverse_string;

///
///
/// the main function
///
fn main() {
    use time_macros::{date, datetime, time};

    println!("Hello, world!");


    let a = 9;
    println!("a is {}", a);

    block::testBlock();
    println!("{:?}", after(datetime!(2000-01-01 0:00)));
    println!("{:?}", reverse("abc"));
    println!("{:?}", reverse("Hello"));

    // after(datetime!(2000-01-01 0:00))
    // datetime!(2000-01-01 0:00);
}


