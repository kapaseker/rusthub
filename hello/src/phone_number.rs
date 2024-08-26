use std::fmt::format;

///NXX NXX-XXXX
/// 
pub fn number(user_number: &str) -> Option<String> {
    let mut phone: Vec<u8> = vec![];

    for x in user_number.chars() {
        if x.is_ascii_digit() {
            phone.push(x.to_digit(10)? as u8)
        }
    }

    if phone.len() == 11 {
        if phone[0] != 1 {
            return None;
        }
        phone.remove(0);
    }

    if phone.len() == 10 {
        if phone[0] > 1 && phone[3] > 1 {
            Some(phone.iter().map(|c| char::from(b'0' + *c)).collect())
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use crate::phone_number::number;

    #[test]

    fn cleans_the_number() {
        let input = "(223) 456-7890";


        let output = number(input);


        let expected = Some("2234567890".into());


        assert_eq!(output, expected);
    }


    #[test]

    fn cleans_numbers_with_dots() {
        let input = "223.456.7890";


        let output = number(input);


        let expected = Some("2234567890".into());


        assert_eq!(output, expected);
    }


    #[test]

    fn cleans_numbers_with_multiple_spaces() {
        let input = "223 456   7890   ";


        let output = number(input);


        let expected = Some("2234567890".into());


        assert_eq!(output, expected);
    }


    #[test]

    fn invalid_when_9_digits() {
        let input = "123456789";


        let output = number(input);


        assert!(output.is_none());
    }


    #[test]

    fn invalid_when_11_digits_does_not_start_with_a_1() {
        let input = "22234567890";


        let output = number(input);


        assert!(output.is_none());
    }


    #[test]

    fn valid_when_11_digits_and_starting_with_1() {
        let input = "12234567890";


        let output = number(input);


        let expected = Some("2234567890".into());


        assert_eq!(output, expected);
    }


    #[test]

    fn valid_when_11_digits_and_starting_with_1_even_with_punctuation() {
        let input = "+1 (223) 456-7890";


        let output = number(input);


        let expected = Some("2234567890".into());


        assert_eq!(output, expected);
    }


    #[test]

    fn invalid_when_more_than_11_digits() {
        let input = "321234567890";


        let output = number(input);


        assert!(output.is_none());
    }


    #[test]

    fn invalid_with_letters() {
        let input = "523-abc-7890";


        let output = number(input);


        assert!(output.is_none());
    }


    #[test]

    fn invalid_with_punctuations() {
        let input = "523-@:!-7890";


        let output = number(input);


        assert!(output.is_none());
    }


    #[test]

    fn invalid_if_area_code_starts_with_0() {
        let input = "(023) 456-7890";


        let output = number(input);


        assert!(output.is_none());
    }


    #[test]

    fn invalid_if_area_code_starts_with_1() {
        let input = "(123) 456-7890";


        let output = number(input);


        assert!(output.is_none());
    }


    #[test]

    fn invalid_if_exchange_code_starts_with_0() {
        let input = "(223) 056-7890";


        let output = number(input);


        assert!(output.is_none());
    }


    #[test]

    fn invalid_if_exchange_code_starts_with_1() {
        let input = "(223) 156-7890";


        let output = number(input);


        assert!(output.is_none());
    }


    #[test]

    fn invalid_if_area_code_starts_with_0_on_valid_11_digit_number() {
        let input = "1 (023) 456-7890";


        let output = number(input);


        assert!(output.is_none());
    }


    #[test]

    fn invalid_if_area_code_starts_with_1_on_valid_11_digit_number() {
        let input = "1 (123) 456-7890";


        let output = number(input);


        assert!(output.is_none());
    }


    #[test]

    fn invalid_if_exchange_code_starts_with_0_on_valid_11_digit_number() {
        let input = "1 (223) 056-7890";


        let output = number(input);


        assert!(output.is_none());
    }


    #[test]

    fn invalid_if_exchange_code_starts_with_1_on_valid_11_digit_number() {
        let input = "1 (223) 156-7890";


        let output = number(input);


        assert!(output.is_none());
    }
}