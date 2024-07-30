fn zeros(n: u64) -> u64 {

    let mut trail_zero = 0;
    let mut n = n;

    while n != 0 {
        n /= 5;
        trail_zero += n;
    }

    trail_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }
}