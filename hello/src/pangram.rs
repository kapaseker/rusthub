/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut char_arr = [0usize;26];
        
    sentence.chars().for_each(|c| {
       if c.is_ascii_alphabetic() { 
           let index = (c.to_ascii_lowercase() as u8 - b'a') as usize;
           char_arr[index] += 1; 
       } 
    });
    
    char_arr.iter().all(|s| *s > 0)
}

mod test {
    use crate::pangram::is_pangram;

    #[test]

    fn empty_sentence() {
        let sentence = "";


        assert!(!is_pangram(sentence));
    }


    #[test]

    fn perfect_lower_case() {
        let sentence = "abcdefghijklmnopqrstuvwxyz";


        assert!(is_pangram(sentence));
    }


    #[test]

    fn only_lower_case() {
        let sentence = "the quick brown fox jumps over the lazy dog";


        assert!(is_pangram(sentence));
    }


    #[test]

    fn missing_the_letter_x() {
        let sentence = "a quick movement of the enemy will jeopardize five gunboats";


        assert!(!is_pangram(sentence));
    }


    #[test]

    fn missing_the_letter_h() {
        let sentence = "five boxing wizards jump quickly at it";


        assert!(!is_pangram(sentence));
    }


    #[test]

    fn with_underscores() {
        let sentence = "the_quick_brown_fox_jumps_over_the_lazy_dog";


        assert!(is_pangram(sentence));
    }


    #[test]

    fn with_numbers() {
        let sentence = "the 1 quick brown fox jumps over the 2 lazy dogs";


        assert!(is_pangram(sentence));
    }


    #[test]

    fn missing_letters_replaced_by_numbers() {
        let sentence = "7h3 qu1ck brown fox jumps ov3r 7h3 lazy dog";


        assert!(!is_pangram(sentence));
    }


    #[test]

    fn mixed_case_and_punctuation() {
        let sentence = "\"Five quacking Zephyrs jolt my wax bed.\"";


        assert!(is_pangram(sentence));
    }


    #[test]

    fn a_m_and_a_m_are_26_different_characters_but_not_a_pangram() {
        let sentence = "abcdefghijklm ABCDEFGHIJKLM";


        assert!(!is_pangram(sentence));
    }
}