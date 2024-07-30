fn solution(s: &str) -> Vec<String> {
    let mut char_vec = s.chars().collect::<Vec<char>>();
    if char_vec.len() & 1 == 1 {
        char_vec.push('_')
    }
    char_vec.chunks(2).map(|s| s.iter().collect()).collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}
