use std::i32;
use crate::break_camel_case::solution;
use crate::convert_string_to_camel_case::to_camel_case;
use crate::fibonacci_streaming::all_fibonacci_numbers;
use crate::gigasecond::after;
use crate::longest_substring_without_repeating_characters::length_of_longest_substring;
use crate::misc::{create_phone_number, find_next_square};
use crate::move_zeroes::move_zeroes;
use crate::reverse_string::reverse;
use crate::prime::{count_primes, is_prime, prime_vector};

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
mod replace_with_alphabet_position;
mod convert_string_to_camel_case;
mod twice_linear;
mod fibonacci_streaming;
mod break_camel_case;
mod perimeter_of_squares_in_a_rectangle;
mod count_ip_addresses;
mod unique_in_order;
mod bob_robot;
mod check_sublist;
mod word_count;
mod convert_matrix;
mod color_choice;
mod multiples_of_3_or_5;
mod split_strings;
mod parse_bank_account_number;
mod snail;
mod ip_validation;
mod find_the_odd_int;
mod equal_sides_of_an_array;
mod duplicate_encoder;
mod not_very_secure;
mod number_of_trailing_zeros_of_n;
mod pete_the_baker;
mod matrix_transpose;
mod sums_of_perfect_squares;
mod smallest_k_elements_of_list;
mod the_sum_of_the_prime_factors_of_a_number;
mod grain;
mod alphabet_war;
mod longest_substring_without_repeating_characters;
mod reverse_words_in_a_string_iii;
mod find_minimum_in_rotated_sorted_array;
mod move_zeroes;
mod sort_colors;
mod increasing_triplet_subsequence;
mod string_compression;
mod kindergarten_garden;
mod allergies;
mod container_with_most_water;
mod max_number_of_k_sum_pairs;
mod maximum_average_subarray_i;
mod maximum_number_of_vowels_in_a_substring_of_given_length;
mod robot_simulator;
mod map_reduce;
mod minesweeper;
mod space_age;
mod acronym;
mod isbn_verifier;
mod knapsack;
mod luhn;
mod roman_numerals;
mod circular_buffer;
mod protein_translation;
mod scrabble_score;

///
///
/// the main function
///
fn main() {
    println!("{}", u8::MAX.saturating_add(2));
    println!("{}", u8::MAX.wrapping_add(2));

    println!("{}", i8::MAX.saturating_add(2));
    println!("{}", i8::MAX.wrapping_add(2));

    // let mut fib_iterator = all_fibonacci_numbers();
    // for _ in 0..10 {
    //     print!("{:?}", fib_iterator.next());
    // }
    // println!();
    // println!("{}", 123u32.count_ones() as usize);
    // println!("{}",to_camel_case("a_b_c"));
    // println!("{}",to_camel_case("the_stealth_warrior"));
    // let mut a = vec![0,1,0,3,12];
    // move_zeroes(&mut a);
    // length_of_longest_substring("pwwkew".to_string());
    // println!("{}",to_camel_case("The-Stealth-Warrior"));
    // println!("{}",solution(&to_camel_case("The-Stealth-Warrior")));
    // println!("{}",to_camel_case("A-B-C"));
    // println!("{}",create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));
    // println!("{}",'a' as u8);
    // println!("{}",'z' as u8);
    // println!("{}",'A' as u8);
    // println!("{}",'Z' as u8);
    // println!("{:?}", count_primes(999900));

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


