
///
/// [无重复字符的最长子串](https://leetcode.cn/problems/longest-substring-without-repeating-characters/description/
///
pub fn length_of_longest_substring(s: String) -> i32 {

    let s :Vec<char>= s.chars().collect();

    let mut l = 0;

    let mut length = 0;
    let mut max_length = 0;

    for r in 1..s.len() {
        for j in l..r {
            if s[j] == s[r] {
                l = j + 1;
                length = r - l;
                break;
            }
        }
        length += 1;
        max_length = length.max(max_length);
    }

    max_length as i32
}

#[cfg(test)]
mod test {
    use crate::longest_substring_without_repeating_characters::length_of_longest_substring;

    #[test]
    fn case_1() {
        assert_eq!(length_of_longest_substring("aaaa".to_string()), 1)
    }

    #[test]
    fn case_2() {
        assert_eq!(length_of_longest_substring("abab".to_string()), 2)
    }

    #[test]
    fn case_3() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3)
    }

    #[test]
    fn case_4() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3)
    }
}