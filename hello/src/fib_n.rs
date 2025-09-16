use num::bigint::BigInt;

fn fib(n: i32) -> BigInt {
    if n == 0 {
        return BigInt::from(0);
    }
    if n > 0 {
        let (a, _) = fib_fast_doubling(n as u32); // ✅ 取第一个值 F(n)
        a
    } else {
        let pos = -n;
        let (a, _) = fib_fast_doubling(pos as u32); // ✅ 取 F(pos)
        if pos % 2 == 1 {
            a
        } else {
            -a
        }
    }
}

fn fib_fast_doubling(n: u32) -> (BigInt, BigInt) {
    if n == 0 {
        return (BigInt::from(0), BigInt::from(1));
    }

    let (a, b) = fib_fast_doubling(n >> 1);
    let c = &a * (&b + &b - &a); // F(2k) = F(k) * [2*F(k+1) - F(k)]
    let d = &a * &a + &b * &b;   // F(2k+1) = F(k)^2 + F(k+1)^2

    if n & 1 == 1 {
        (d.clone(), c + &d) // (F(2k+1), F(2k+2))
    } else {
        (c, d) // (F(2k), F(2k+1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(0), BigInt::from(0));
        assert_eq!(fib(1), BigInt::from(1));
        assert_eq!(fib(2), BigInt::from(1));
        assert_eq!(fib(3), BigInt::from(2));
        assert_eq!(fib(10), BigInt::from(55));
        assert_eq!(fib(-1), BigInt::from(1));
        assert_eq!(fib(-2), BigInt::from(-1));
        assert_eq!(fib(-3), BigInt::from(2));
        assert_eq!(fib(-10), BigInt::from(-55));
    }
}
