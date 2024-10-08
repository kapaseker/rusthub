use std::fmt::Display;
use crate::luhn::is_valid;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
// impl<'a> Luhn for &'a str {
//     fn valid_luhn(&self) -> bool {
//         is_valid_luhn(self)
//     }
// }
// 
// impl Luhn for String {
//     fn valid_luhn(&self) -> bool {
//         is_valid_luhn(&self)
//     }
// }

impl<T> Luhn for T 
    where T : Display,
{
    fn valid_luhn(&self) -> bool {
        is_valid(&format!("{}", self))
    }
}

#[cfg(test)]
mod test {
    use crate::luhn_trait::*;

    #[test]
    
    fn you_can_validate_from_a_str() {
        assert!("046 454 286".valid_luhn());
    
    
        assert!(!"046 454 287".valid_luhn());
    }
    
    
    #[test]
    
    fn you_can_validate_from_a_string() {
        assert!(String::from("046 454 286").valid_luhn());
    
    
        assert!(!String::from("046 454 287").valid_luhn());
    }


    #[test]

    fn you_can_validate_from_a_u8() {
        assert!(240u8.valid_luhn());


        assert!(!241u8.valid_luhn());
    }


    #[test]

    fn you_can_validate_from_a_u16() {
        let valid = 64_436u16;


        let invalid = 64_437u16;


        assert!(valid.valid_luhn());


        assert!(!invalid.valid_luhn());
    }


    #[test]

    fn you_can_validate_from_a_u32() {
        let valid = 46_454_286u32;


        let invalid = 46_454_287u32;


        assert!(valid.valid_luhn());


        assert!(!invalid.valid_luhn());
    }


    #[test]

    fn you_can_validate_from_a_u64() {
        let valid = 8273_1232_7352_0562u64;


        let invalid = 8273_1232_7352_0569u64;


        assert!(valid.valid_luhn());


        assert!(!invalid.valid_luhn());
    }


    #[test]

    fn you_can_validate_from_a_usize() {
        let valid = 8273_1232_7352_0562usize;


        let invalid = 8273_1232_7352_0569usize;


        assert!(valid.valid_luhn());


        assert!(!invalid.valid_luhn());
    }


    #[test]
    
    fn input_digit_9_is_still_correctly_converted_to_output_digit_9() {
        assert!("091".valid_luhn());
    }
}