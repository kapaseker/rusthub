
///
/// [定长子串中元音的最大数目](https://leetcode.cn/problems/maximum-number-of-vowels-in-a-substring-of-given-length)
///
pub fn max_vowels(s: String, k: i32) -> i32 {
    let k = k as usize;
    let chars: Vec<char> = s.chars().collect();

    let mut sum = 0;
    for x in chars[0..k].iter() {
        if matches!(*x,'a'|'e'|'i'|'o'|'u') {
            sum += 1;
        }
    }

    if sum == k {
        return sum as i32;
    }

    let mut max = sum;

    for i in 1..=(chars.len() - k) {
        if matches!(chars[i - 1],'a'|'e'|'i'|'o'|'u') {
            sum -= 1;
        }
        if matches!(chars[i + k - 1],'a'|'e'|'i'|'o'|'u') {
            sum += 1;
        }

        max = max.max(sum);

        if max == k {
            break;
        }
    }

    max as i32
}

#[cfg(test)]
mod test {
    use crate::maximum_number_of_vowels_in_a_substring_of_given_length::max_vowels;

    #[test]
    fn case_1() {
        assert_eq!(3, max_vowels("abciiidef".to_string(), 3))
    }

    #[test]
    fn case_2() {
        assert_eq!(2, max_vowels("aeiou".to_string(), 2))
    }

    #[test]
    fn case_3() {
        assert_eq!(0, max_vowels("rhythms".to_string(), 4))
    }

    #[test]
    fn case_4() {
        assert_eq!(4, max_vowels("lloveyou".to_string(), 7))
    }
}