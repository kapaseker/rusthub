pub fn perimeter(n: u64) -> u64 {
    let (mut a, mut b) = (1u64, 1u64);

    let mut step = 0;
    let mut sum = 0;
    loop {
        sum += a;
        let t = a + b;
        a = b;
        b = t;
        step += 1;

        if step > n {
            break;
        }
    }

    sum * 4
}

mod test {
    use crate::perimeter_of_squares_in_a_rectangle::perimeter;

    #[test]
    fn case_1() {
        assert_eq!(perimeter(1), 8);
        assert_eq!(perimeter(2), 16);
        assert_eq!(perimeter(5), 80);
    }
}