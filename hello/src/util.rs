pub fn is_prime(n: u32) -> bool {
    if n <= 1 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }

    let end = (n as f32).sqrt().round() as u32;

    for i in (5..=end).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
    }

    return true;
}