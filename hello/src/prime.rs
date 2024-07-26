
///检查一个数，是否是质数
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

///生成质数的数列
pub fn prime_sequence(n: u32) -> Vec<u32> {

    let mut primes = vec![];

    if n == 0 { return primes }

    let mut primes_check: Vec<bool> = vec![true; n as usize];
    let mut i = 2;
    while i < n {
        let iu = i as usize;
        if primes_check[iu] {
            primes.push(i);
            let iuu = i as u64;
            if iuu * iuu < n as u64 {
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
    prime_sequence(n as u32).len() as i32
}