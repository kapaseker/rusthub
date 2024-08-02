
///
/// [反转字符串中的单词 III](https://leetcode.cn/problems/reverse-words-in-a-string-iii)
///
pub fn reverse_words(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut last_char: usize = 0;
    for i in 1..len {
        let c = &chars[i];
        if *c == ' ' {
            chars[last_char..i].reverse();
            last_char = i + 1;
        }
    }

    chars[last_char..len].reverse();

    String::from_iter(chars)
}

#[cfg(test)]
mod test {
    use super::reverse_words;
    #[test]
    fn case_1() {
        assert_eq!(reverse_words("Let's take LeetCode contest".to_string()), "s'teL ekat edoCteeL tsetnoc".to_string())
    }

    #[test]
    fn case_2() {
        assert_eq!(reverse_words("Mr Ding".to_string()), "rM gniD".to_string())
    }
}