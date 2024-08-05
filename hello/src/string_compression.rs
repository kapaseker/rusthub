
///
/// [压缩字符串](https://leetcode.cn/problems/string-compression)
///
pub fn compress(chars: &mut Vec<char>) -> i32 {
    let mut cur: usize = 0;
    let mut last_char = chars[0];
    let mut last_count: usize = 1;
    let end = chars.len();

    for i in 1..end {
        if chars[i] != last_char {
            chars[cur] = last_char;
            char_count(chars, &mut cur, last_count);
            cur += 1;
            last_char = chars[i];
            last_count = 1;
        } else {
            last_count += 1;
        }
    }

    chars[cur] = last_char;
    char_count(chars, &mut cur, last_count);

    (cur + 1) as i32
}

fn char_count(mut chars: &mut Vec<char>, cur: &mut usize, last_count: usize) {
    let mut last_count = last_count;
    if last_count > 1 {
        let start = *cur + 1;
        while last_count != 0 {
            *cur += 1;
            let c = last_count % 10;
            chars[*cur] = char::from('0' as u8 + c as u8);
            last_count /= 10;
        }
        chars[start..=*cur].reverse()
    }
}

#[cfg(test)]
mod test {
    use crate::string_compression::compress;

    #[test]
    fn case_1() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let count = compress(&mut chars);
        let result_chars: Vec<char> = chars.into_iter().take(count as usize).collect();
        assert_eq!(result_chars, vec!['a', '2', 'b', '2', 'c', '3']);
        assert_eq!(count, 6);
    }

    #[test]
    fn case_2() {
        let mut chars = vec!['a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'];
        let count = compress(&mut chars);
        let result_chars: Vec<char> = chars.into_iter().take(count as usize).collect();
        assert_eq!(result_chars, vec!['a', 'b', '1', '2']);
        assert_eq!(count, 4);
    }

    #[test]
    fn case_3() {
        let mut chars = vec!['a'];
        let count = compress(&mut chars);
        let result_chars: Vec<char> = chars.into_iter().take(count as usize).collect();
        assert_eq!(result_chars, vec!['a']);
        assert_eq!(count, 1);
    }

    #[test]
    fn case_4() {
        let mut chars = vec!['a', 'b'];
        let count = compress(&mut chars);
        let result_chars: Vec<char> = chars.into_iter().take(count as usize).collect();
        assert_eq!(result_chars, vec!['a', 'b']);
        assert_eq!(count, 2);
    }
}