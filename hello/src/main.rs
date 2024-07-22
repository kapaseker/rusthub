use crate::gigasecond::after;
use crate::reverse_string::reverse;
use crate::util::is_prime;

mod block;
mod gigasecond;
mod reverse_string;
mod beer_song;
mod util;
mod nth_prime;
mod difference_of_squares;

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

    println!("{:?}", -5 / 3);
    println!("{:?}", -5 % 3);
    println!("{:?}", (-59i32).div_euclid(3i32));
    println!("{:?}", (-5i32).rem_euclid(3i32));

    println!("----------------");

    println!("{:?}", (-59i32).rem_euclid(60i32));
    println!("{:?}", (-60i32).rem_euclid(60i32));

    println!("----------------");

    println!("{:?}", is_prime(7u32));
    println!("{:?}", is_prime(13u32));
    println!("{:?}", is_prime(12u32));
    println!("{:?}", is_prime(15u32));
    println!("{:?}", is_prime(27u32));
    println!("{:?}", is_prime(11u32));

    // after(datetime!(2000-01-01 0:00))
    // datetime!(2000-01-01 0:00);
}


