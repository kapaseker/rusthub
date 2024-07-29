pub fn find_next_square(sq: u64) -> Option<u64> {
    let root = (sq as f64).sqrt();
    (root.trunc() == root).then(|| (root as u64 + 1).pow(2))
}

pub fn create_phone_number(numbers: &[u8]) -> String {
    // format!("({}) {}-{}",&numbers[0..3],&numbers[3..6],&numbers[6..])
    format!("({}{}{}) {}{}{}-{}{}{}{}", &numbers[0], &numbers[1], &numbers[2], &numbers[3], &numbers[4], &numbers[5], &numbers[6], &numbers[7], &numbers[8], &numbers[9])
}
