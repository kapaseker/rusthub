use crate::prime::prime_factors;

fn mult_primefactor_sum(a: u32, b: u32) -> Vec<u32> {
    let mut nums = vec![];
    for x in a..=b {
        let a = prime_factors(x as u64);
        if a.len() > 1 {
            let sum = a.iter().sum::<u64>() as u32;
            if x % sum == 0 {
                nums.push(x);
            }
        }
    }
    nums
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::mult_primefactor_sum;

    fn dotest(a: u32, b: u32, expected: &[u32]) {
        let actual = mult_primefactor_sum(a, b);
        assert!(actual == expected,
                "With a = {a}, b = {b}\nExpected {expected:?} but got {actual:?}")
    }

    #[test]
    fn fixed_tests() {
        dotest(10, 100, &[16, 27, 30, 60, 70, 72, 84]);
        dotest(80, 150, &[84, 105, 150]);
    }
}