pub fn find_next_square(sq: u64) -> Option<u64> {
    let root = (sq as f64).sqrt();
    (root.trunc() == root).then(|| (root as u64 + 1).pow(2))
}