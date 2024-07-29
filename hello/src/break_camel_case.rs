use regex::Regex;

pub fn solution(s: &str) -> String {
    Regex::new(r"(?P<head>[A-Z])")
        .unwrap()
        .replace_all(s," $head").to_string()
}