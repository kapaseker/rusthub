fn main() {}

pub fn encode(source: &str) -> String {
    let mut chars = source.chars();
    let mut count = 1;
    let mut str = String::new();

    let mut ch = match chars.next() {
        Some(c) => c,
        None => return str,
    };

    for c in chars {
        if c == ch {
            count += 1;
        } else {
            if count > 1 {
                str.push_str(&count.to_string());
            }
            str.push(ch);
            ch = c;
            count = 1;
        }
    }

    if count > 1 {
        str.push_str(&count.to_string());
    }
    str.push(ch);

    str
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut count_str = String::new();

    for c in source.chars() {
        if c.is_ascii_digit() {
            count_str.push(c);
        } else {
            let count = if count_str.is_empty() {
                1
            } else {
                count_str.parse().unwrap_or(1)
            };

            for _ in 0..count {
                result.push(c);
            }

            count_str.clear();
        }
    }

    result
}

mod test {

    use crate::run_len::decode;
    use crate::run_len::encode;

    #[test]
    fn encode_empty_string() {
        let input = "";

        let output = encode(input);

        let expected = "";

        assert_eq!(output, expected);
    }

    #[test]
    fn encode_single_characters_only_are_encoded_without_count() {
        let input = "XYZ";

        let output = encode(input);

        let expected = "XYZ";

        assert_eq!(output, expected);
    }

    #[test]
    fn encode_string_with_no_single_characters() {
        let input = "AABBBCCCC";

        let output = encode(input);

        let expected = "2A3B4C";

        assert_eq!(output, expected);
    }

    #[test]
    fn encode_single_characters_mixed_with_repeated_characters() {
        let input = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB";

        let output = encode(input);

        let expected = "12WB12W3B24WB";

        assert_eq!(output, expected);
    }

    #[test]
    fn encode_multiple_whitespace_mixed_in_string() {
        let input = "  hsqq qww  ";

        let output = encode(input);

        let expected = "2 hs2q q2w2 ";

        assert_eq!(output, expected);
    }

    #[test]
    fn encode_lowercase_characters() {
        let input = "aabbbcccc";

        let output = encode(input);

        let expected = "2a3b4c";

        assert_eq!(output, expected);
    }

    #[test]
    fn decode_empty_string() {
        let input = "";

        let output = decode(input);

        let expected = "";

        assert_eq!(output, expected);
    }

    #[test]
    fn decode_single_characters_only() {
        let input = "XYZ";

        let output = decode(input);

        let expected = "XYZ";

        assert_eq!(output, expected);
    }

    #[test]
    fn decode_string_with_no_single_characters() {
        let input = "2A3B4C";

        let output = decode(input);

        let expected = "AABBBCCCC";

        assert_eq!(output, expected);
    }

    #[test]
    fn decode_single_characters_with_repeated_characters() {
        let input = "12WB12W3B24WB";

        let output = decode(input);

        let expected = "WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWB";

        assert_eq!(output, expected);
    }

    #[test]
    fn decode_multiple_whitespace_mixed_in_string() {
        let input = "2 hs2q q2w2 ";

        let output = decode(input);

        let expected = "  hsqq qww  ";

        assert_eq!(output, expected);
    }

    #[test]
    fn decode_lowercase_string() {
        let input = "2a3b4c";

        let output = decode(input);

        let expected = "aabbbcccc";

        assert_eq!(output, expected);
    }

    #[test]
    fn consistency_encode_followed_by_decode_gives_original_string() {
        let input = "zzz ZZ  zZ";

        let output = decode(&encode(input));

        let expected = "zzz ZZ  zZ";

        assert_eq!(output, expected);
    }
}
