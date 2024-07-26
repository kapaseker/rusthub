use crate::gigasecond::after;
use crate::misc::find_next_square;
use crate::reverse_string::reverse;
use crate::prime::{count_primes, is_prime, prime_sequence};

mod block;
mod gigasecond;
mod reverse_string;
mod beer_song;
mod prime;
mod nth_prime;
mod difference_of_squares;
mod high_scores;
mod misc;
mod anagram;
mod prime_factors;
mod reverse_vowels_of_a_string;

///
///
/// the main function
///
fn main() {

    // println!("{}", 123u32.count_ones() as usize);

    println!("{:?}", count_primes(999900));

    // println!("{:?}", find_next_square(144u64));
    //
    // println!("{}",'Γ'.to_ascii_lowercase() == 'Γ');
    //
    // let s = "Δ-Straẞe-İΒΓΑ,BA";
    // println!("{}", s.chars().flat_map(|c| c.to_lowercase()).collect::<String>());

    // use time_macros::{date, datetime, time};
    //
    // println!("Hello, world!");
    //
    //
    // let a = 9;
    // println!("a is {}", a);
    //
    // block::testBlock();
    // println!("{:?}", after(datetime!(2000-01-01 0:00)));
    // println!("{:?}", reverse("abc"));
    // println!("{:?}", reverse("Hello"));
    //
    // println!("{:?}", -5 / 3);
    // println!("{:?}", -5 % 3);
    // println!("{:?}", (-59i32).div_euclid(3i32));
    // println!("{:?}", (-5i32).rem_euclid(3i32));
    //
    // println!("----------------");
    //
    // println!("{:?}", (-59i32).rem_euclid(60i32));
    // println!("{:?}", (-60i32).rem_euclid(60i32));
    //
    // println!("----------------");
    //
    // println!("{:?}", is_prime(7u32));
    // println!("{:?}", is_prime(13u32));
    // println!("{:?}", is_prime(12u32));
    // println!("{:?}", is_prime(15u32));
    // println!("{:?}", is_prime(27u32));
    // println!("{:?}", is_prime(11u32));

    // after(datetime!(2000-01-01 0:00))
    // datetime!(2000-01-01 0:00);
}


