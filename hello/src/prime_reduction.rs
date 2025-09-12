fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let limit = (n as f64).sqrt() as u32 + 1;
    for i in (3..limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn digit_square_sum(mut n: u32) -> u32 {
    let mut sum = 0;
    while n > 0 {
        let digit = n % 10;
        sum += digit * digit;
        n /= 10;
    }
    sum
}

fn ends_in_one(mut n: u32) -> bool {
    // 使用快慢指针检测循环（Floyd判圈算法）
    // 或者使用集合记录访问过的数字
    let mut seen = std::collections::HashSet::new();

    while n != 1 && !seen.contains(&n) {
        seen.insert(n);
        n = digit_square_sum(n);
    }

    n == 1
}

fn solve(start: u32, end: u32) -> usize {
    let mut count = 0;

    for n in start..end {
        if is_prime(n) && ends_in_one(n) {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_square_sum() {
        assert_eq!(digit_square_sum(23), 13);
        assert_eq!(digit_square_sum(13), 10);
        assert_eq!(digit_square_sum(10), 1);
        assert_eq!(digit_square_sum(7), 49);
    }

    #[test]
    fn test_ends_in_one() {
        assert!(ends_in_one(23));
        assert!(ends_in_one(7));
        assert!(ends_in_one(13));
        assert!(ends_in_one(10));
        assert!(ends_in_one(1));

        // 测试一个不会到达1的数（例如89，它会进入循环）
        assert!(!ends_in_one(89));
    }

    #[test]
    fn test_is_prime() {
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(5));
        assert!(!is_prime(1));
        assert!(is_prime(23));
        assert!(!is_prime(25));
    }

    #[test]
    fn test_example_range() {
        // 范围 [2, 25) 内的质数：2,3,5,7,11,13,17,19,23
        // 需要检查哪些最终会变成1
        assert_eq!(solve(2, 25), 5);
        // 应该是：7, 11, 13, 19, 23（5个）
        // 2 -> 4 -> 16 -> 37 -> 58 -> 89 -> ...（进入89循环，不会到1）
        // 3 -> 9 -> 81 -> 65 -> 61 -> 37 -> ...（同上）
        // 5 -> 25 -> 29 -> 85 -> 89 -> ...（同上）
        // 17 -> 50 -> 25 -> ...（同上）
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(solve(1, 2), 0);
        assert_eq!(solve(2, 3), 0); // 2不会到1
        assert_eq!(solve(7, 8), 1); // 7会到1
    }
}

fn main() {
    // 示例：计算范围 [2, 25) 内符合条件的质数个数
    let result = solve(2, 25);
    println!("在范围 [2, 25) 内，有 {} 个质数最终会变成 1", result);

    // 计算更大的范围，比如 [2, 100)
    let result2 = solve(2, 100);
    println!("在范围 [2, 100) 内，有 {} 个质数最终会变成 1", result2);

    // 验证上限50000（实际运行可能需要一点时间）
    // let result3 = solve_primes_ending_in_one(2, 50001);
    // println!("在范围 [2, 50001) 内，有 {} 个质数最终会变成 1", result3);
}