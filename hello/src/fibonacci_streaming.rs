use num::BigUint;

struct Fibonacci(BigUint, BigUint);

impl Iterator for Fibonacci {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {

        let cur = self.0.clone();

        self.0 = self.1.clone();
        self.1 = cur.clone() + self.1.clone();

        Some(cur)
    }
}
///[Fibonacci Streaming](https://www.codewars.com/kata/55695bc4f75bbaea5100016b)
pub fn all_fibonacci_numbers() -> impl Iterator<Item = BigUint> {
    Fibonacci(BigUint::from(1u32), BigUint::from(1u32))
}