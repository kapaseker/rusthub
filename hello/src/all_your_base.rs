#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    
    if from_base < 2 { 
        return Err(Error::InvalidInputBase);
    }
    
    if to_base < 2 { 
        return Err(Error::InvalidOutputBase);
    }
    
    let mut sum = 0u32;
    let mut len = number.len() as u32;
    for x in number {
        if *x < from_base {
            sum += x * from_base.pow(len - 1);
            len -= 1;
        } else { 
            return Err(Error::InvalidDigit(*x));
        }
    }
    
    let mut digits = vec![];

    loop {
        digits.push(sum % to_base);
        sum /= to_base;
        
        if sum == 0 { 
            break;
        }
    }
    
    digits.reverse();
    
    Ok(digits) 
}

mod test {
    use crate::all_your_base::{convert, Error};

    #[test]

    fn single_bit_one_to_decimal() {
        let input_base = 2;


        let input_digits = &[1];


        let output_base = 10;


        let output_digits = vec![1];


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }


    #[test]

    fn binary_to_single_decimal() {
        let input_base = 2;


        let input_digits = &[1, 0, 1];


        let output_base = 10;


        let output_digits = vec![5];


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }


    #[test]

    fn single_decimal_to_binary() {
        let input_base = 10;


        let input_digits = &[5];


        let output_base = 2;


        let output_digits = vec![1, 0, 1];


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }


    #[test]

    fn binary_to_multiple_decimal() {
        let input_base = 2;


        let input_digits = &[1, 0, 1, 0, 1, 0];


        let output_base = 10;


        let output_digits = vec![4, 2];


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }


    #[test]

    fn decimal_to_binary() {
        let input_base = 10;


        let input_digits = &[4, 2];


        let output_base = 2;


        let output_digits = vec![1, 0, 1, 0, 1, 0];


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }


    #[test]

    fn trinary_to_hexadecimal() {
        let input_base = 3;


        let input_digits = &[1, 1, 2, 0];


        let output_base = 16;


        let output_digits = vec![2, 10];


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }


    #[test]

    fn hexadecimal_to_trinary() {
        let input_base = 16;


        let input_digits = &[2, 10];


        let output_base = 3;


        let output_digits = vec![1, 1, 2, 0];


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }


    #[test]

    fn test_15_bit_integer() {
        let input_base = 97;


        let input_digits = &[3, 46, 60];


        let output_base = 73;


        let output_digits = vec![6, 10, 45];


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }


    #[test]

    fn empty_list() {
        let input_base = 2;


        let input_digits = &[];


        let output_base = 10;


        let output_digits = vec![0];


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }


    #[test]

    fn single_zero() {
        let input_base = 10;


        let input_digits = &[0];


        let output_base = 2;


        let output_digits = vec![0];


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }


    #[test]

    fn multiple_zeros() {
        let input_base = 10;


        let input_digits = &[0, 0, 0];


        let output_base = 2;


        let output_digits = vec![0];


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }


    #[test]

    fn leading_zeros() {
        let input_base = 7;


        let input_digits = &[0, 6, 0];


        let output_base = 10;


        let output_digits = vec![4, 2];


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }


    #[test]

    fn input_base_is_one() {
        let input_base = 1;


        let input_digits = &[0];


        let output_base = 10;


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidInputBase)
        );
    }


    #[test]

    fn input_base_is_zero() {
        let input_base = 0;


        let input_digits = &[];


        let output_base = 10;


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidInputBase)
        );
    }


    #[test]

    fn invalid_positive_digit() {
        let input_base = 2;


        let input_digits = &[1, 2, 1, 0, 1, 0];


        let output_base = 10;


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidDigit(2))
        );
    }


    #[test]

    fn output_base_is_one() {
        let input_base = 2;


        let input_digits = &[1, 0, 1, 0, 1, 0];


        let output_base = 1;


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidOutputBase)
        );
    }


    #[test]

    fn output_base_is_zero() {
        let input_base = 10;


        let input_digits = &[7];


        let output_base = 0;


        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidOutputBase)
        );
    }
}