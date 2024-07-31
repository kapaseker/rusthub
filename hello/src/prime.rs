///检查一个数，是否是质数
pub fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }

    let end = (n as f32).sqrt().round() as u64;

    for i in (5..=end).step_by(6) {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
    }

    return true;
}

///
/// 质因数分解
/// [Prime Factors](https://www.codewars.com/kata/542f3d5fd002f86efc00081a)
///
pub fn prime_factors(n: u64) -> Vec<u64> {
    if n <= 1 { return vec![]; }

    let mut i = 2u64;
    let mut n = n;
    let mut factors = Vec::new();

    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 1;
    }

    ///如果入参是个质数，那么上面的循环是不会走的，直接走这里，自己就是质因数
    if n != 1 {
        factors.push(n);
    }

    factors
}

///
/// 生成小于N质数的数列
///
pub fn prime_vector(n: u64) -> Vec<u64> {
    let mut primes = vec![];

    if n == 0 { return primes; }

    let mut primes_check: Vec<bool> = vec![true; n as usize];
    let mut i = 2;
    while i < n {
        let iu = i as usize;
        if primes_check[iu] {
            primes.push(i);
            let iuu = i;
            if iuu * iuu < n {
                for j in (i * i..n).step_by(iu) {
                    primes_check[j as usize] = false
                }
            }
        }
        i += 1;
    }

    primes
}


///
/// [计数质数](https://leetcode.cn/problems/count-primes/description/)
///
pub fn count_primes(n: i32) -> i32 {
    prime_vector(n as u64).len() as i32
}