use std::collections::HashMap;

fn duplicate_encode(word: &str) -> String {
    let mut char_map: HashMap<char, u32> = word.chars().fold(HashMap::new(), |mut map, value| {
        *map.entry(value.to_ascii_lowercase()).or_default() += 1;
        map
    });

    word.chars().map(|s| if char_map[&(s.to_ascii_lowercase())] > 1 { ')' } else { '(' } ).collect()
}


#[cfg(test)]
mod test {
    use crate::duplicate_encoder::duplicate_encode;

    #[test]
    fn run_tests() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
        assert_eq!(duplicate_encode("(( @"), "))((");
    }
}

