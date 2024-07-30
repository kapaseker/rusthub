use num::{One};

/// 该方法有个非常厉害的地方，避免的数字溢出的问题
/// [Color Choice](https://www.codewars.com/kata/55be10de92aad5ef28000023)
///
pub fn check_choose(m: u64, n: u64) -> i64 {
    let mut result: u64 = 1;
    for i in 0..n + 1 {
        if result == m { return i as i64; };
        result = result * (n - i) / (i + 1);
    }
    -1
}

mod test {
    use crate::color_choice::check_choose;

    fn dotest(m: u64, n: u64, exp: i64) -> () {
        assert_eq!(check_choose(m, n), exp)
    }

    #[test]
    fn basics_check_choose() {
        dotest(6, 4, 2);
        dotest(4, 4, 1);
        dotest(35, 7, 3);
        dotest(36, 7, -1);
        dotest(184756, 20, 10);
        dotest(47129212243960, 50, 20);
    }
}