pub fn reverse_vowels(s: String) -> String {

    let mut chars: Vec<char> = s.chars().collect();

    let mut i = 0;
    let mut j = chars.len() - 1;

    while i < j {

        if !matches!(chars[i],'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U') {
            i += 1;
            continue;
        }

        if !matches!(chars[j],'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U') {
            j -= 1;
            continue;
        }

        chars.swap(i,j);

        i += 1;
        j -= 1;
    }

    String::from_iter(chars)
}