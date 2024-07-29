use regex::Regex;

pub fn to_camel_case(text: &str) -> String {
    Regex::new(r"[_-]")
        .unwrap()
        .split(text)
        .enumerate()
        .map(|(i, x)| {
            if i != 0usize {
                let mut s = x.to_string();
                if let Some(r) = s.get_mut(0..1) {
                    r.make_ascii_uppercase()
                }
                s
            } else {
                x.to_string()
            }
        })
        .collect()
}
