use std::collections::HashMap;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let protein_map = HashMap::from([
        ('D', 2u64),
        ('G', 2),
        ('B', 3),
        ('C', 3),
        ('M', 3),
        ('P', 3),
        ('F',4),
        ('H',4),
        ('V',4),
        ('W',4),
        ('Y',4),
        ('K',5),
        ('J',8),
        ('X',8),
        ('Q',10),
        ('Z',10),
    ]);

    word.chars().fold(0,|s,c| {
        s + if c.is_ascii()  { 
             *protein_map.get(&c.to_ascii_uppercase()).unwrap_or(&1) 
        }else {
            0u64 
        }
    })
}


mod test {
    use crate::scrabble_score::score;

    #[test]

    fn lowercase_letter() {
        let input = "a";


        let output = score(input);


        let expected = 1;


        assert_eq!(output, expected);
    }


    #[test]
    

    fn uppercase_letter() {
        let input = "A";


        let output = score(input);


        let expected = 1;


        assert_eq!(output, expected);
    }


    #[test]
    

    fn valuable_letter() {
        let input = "f";


        let output = score(input);


        let expected = 4;


        assert_eq!(output, expected);
    }


    #[test]
    

    fn short_word() {
        let input = "at";


        let output = score(input);


        let expected = 2;


        assert_eq!(output, expected);
    }


    #[test]
    

    fn short_valuable_word() {
        let input = "zoo";


        let output = score(input);


        let expected = 12;


        assert_eq!(output, expected);
    }


    #[test]
    

    fn medium_word() {
        let input = "street";


        let output = score(input);


        let expected = 6;


        assert_eq!(output, expected);
    }


    #[test]
    

    fn medium_valuable_word() {
        let input = "quirky";


        let output = score(input);


        let expected = 22;


        assert_eq!(output, expected);
    }


    #[test]
    

    fn long_mixed_case_word() {
        let input = "OxyphenButazone";


        let output = score(input);


        let expected = 41;


        assert_eq!(output, expected);
    }


    #[test]
    

    fn english_like_word() {
        let input = "pinata";


        let output = score(input);


        let expected = 8;


        assert_eq!(output, expected);
    }


    #[test]
    

    fn empty_input() {
        let input = "";


        let output = score(input);


        let expected = 0;


        assert_eq!(output, expected);
    }


    #[test]
    

    fn entire_alphabet_available() {
        let input = "abcdefghijklmnopqrstuvwxyz";


        let output = score(input);


        let expected = 87;


        assert_eq!(output, expected);
    }


    #[test]
    

    fn non_english_scrabble_letters_do_not_score() {
        let input = "piñata";


        let output = score(input);


        let expected = 7;


        assert_eq!(output, expected);
    }


    #[test]
    

    fn german_letters_do_not_score() {
        let input = "STRAßE";


        let output = score(input);


        let expected = 5;


        assert_eq!(output, expected);
    }
}