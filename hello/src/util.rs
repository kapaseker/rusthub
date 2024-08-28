///
/// what is c-n-k. select k elements in n elements.
/// it represents: ![nk](url:https://bkimg.cdn.bcebos.com/formula/c94512a2945a02b6f9a6a2b78bd32679.svg)\
///
pub fn cn_k(n: u32, k: u32) -> u32 {
    if k >= n {
        panic!("n must bigger than k");
    }

    let mut factorial_n = 1;
    let mut n = n;
    let i = n - k;
    while n > i {
        factorial_n *= n;
        n -= 1;
    }

    let mut factorial_k = 1;
    let mut k = k;
    while k > 1 {
        factorial_k *= k;
        k -= 1;
    }

    factorial_n / factorial_k
}