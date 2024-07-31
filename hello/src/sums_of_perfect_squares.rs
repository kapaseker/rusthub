use num::integer::Roots;

fn sum_of_squares(n: u64) -> u64 {
    if is_perfect_square_number(n) { return 1; }
    if is_4k_number(n) { return 4; }

    for i in 1..=n.sqrt() {
        if is_perfect_square_number(n - i * i) {
            return 2;
        }
    }

    return 3;
}

///
/// 当 n=4k×(8m+7) 时，n 只能被表示为四个正整数的平方和
///
fn is_4k_number(n: u64) -> bool {
    let mut n = n;
    while n % 4 == 0 {
        n /= 4;
    }
    n % 8 == 7
}

fn is_perfect_square_number(n: u64) -> bool {
    n.sqrt().pow(2) == n
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::sum_of_squares;


    #[test]
    fn sample_tests() {
        let a :i32 = 45;
        (a as f64).sqrt().trunc();
        assert_eq!(sum_of_squares(15), 4, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(sum_of_squares(16), 1, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(sum_of_squares(17), 2, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(sum_of_squares(18), 2, "\nYour answer (left) is not the expected answer (right).");
        assert_eq!(sum_of_squares(19), 3, "\nYour answer (left) is not the expected answer (right).");
    }
}
