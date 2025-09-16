fn consec_kprimes(k: i32, arr: Vec<i32>) -> i32 {
    if arr.len() < 2 || k < 0 {
        return 0;
    }

    let k = k as usize; // 转为 usize，质因数个数不可能为负
    let mut count = 0;

    for i in 0..arr.len() - 1 {
        let a = count_prime_factors(arr[i]);
        let b = count_prime_factors(arr[i + 1]);
        if a == k && b == k {
            count += 1;
        }
    }

    count as i32
}

fn count_prime_factors(mut n: i32) -> usize {
    if n <= 1 {
        return 0;
    }
    let mut count = 0;
    let mut d = 2;
    let mut n = n as usize; // 转为无符号，避免负数问题（题目说正整数）

    while d * d <= n {
        while n % d == 0 {
            count += 1;
            n /= d;
        }
        d += 1;
    }
    if n > 1 {
        count += 1;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_case_1() {
        // 题目给出的例子1
        let arr = vec![10005, 10030, 10026, 10008, 10016, 10028, 10004];
        assert_eq!(consec_kprimes(4, arr), 3);
    }

    #[test]
    fn test_example_case_2() {
        // 题目给出的例子2
        let arr = vec![10175, 10185, 10180, 10197];
        assert_eq!(consec_kprimes(4, arr), 3);
    }

    #[test]
    fn test_no_consecutive_kprimes() {
        // 没有任何相邻对满足条件
        let arr = vec![2, 4, 3, 9]; // 1-prime, 2-prime, 1-prime, 2-prime
        assert_eq!(consec_kprimes(2, arr), 0); // 没有连续两个都是 2-prime
    }

    #[test]
    fn test_all_consecutive_kprimes() {
        // 所有相邻对都满足
        let arr = vec![4, 6, 9, 10]; // 全是 2-prime
        assert_eq!(consec_kprimes(2, arr), 3); // (4,6), (6,9), (9,10)
    }

    #[test]
    fn test_edge_cases() {
        // 边界情况：空数组、单元素、k为负
        assert_eq!(consec_kprimes(2, vec![]), 0);
        assert_eq!(consec_kprimes(2, vec![4]), 0);
        assert_eq!(consec_kprimes(-1, vec![4, 6]), 0); // k 为负 → 0
        assert_eq!(consec_kprimes(0, vec![1, 1]), 1); // 1 是 0-prime，相邻匹配
    }

    #[test]
    fn test_k1_primes_consecutive() {
        // k=1：质数相邻对
        let arr = vec![2, 3, 5, 7, 11];
        assert_eq!(consec_kprimes(1, arr), 4); // 每一对相邻质数都算
    }
}