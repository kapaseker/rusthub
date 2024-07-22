pub fn square_of_sum(n: u32) -> u32 {
    return ((n + 1) * n / 2).pow(2);
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut n = n;
    let mut sum = 0u32;
    while n != 1 {
        sum += n.pow(2);
        n -= 1;
    }
    sum + 1
}

pub fn difference(n: u32) -> u32 {
    let mut sum = 0;
    for i in 0..n {
        for j in (i + 1)..=n {
            sum += i * j * 2;
        }
    }
    sum
}


mod test {
    use crate::difference_of_squares as squares;

    #[test]
    fn square_of_sum_1() {
        assert_eq!(1, squares::square_of_sum(1));
    }


    #[test]
    #[ignore]
    fn square_of_sum_5() {
        assert_eq!(225, squares::square_of_sum(5));
    }


    #[test]
    #[ignore]
    fn square_of_sum_100() {
        assert_eq!(25_502_500, squares::square_of_sum(100));
    }


    #[test]
    #[ignore]
    fn sum_of_squares_1() {
        assert_eq!(1, squares::sum_of_squares(1));
    }


    #[test]
    #[ignore]
    fn sum_of_squares_5() {
        assert_eq!(55, squares::sum_of_squares(5));
    }


    #[test]
    #[ignore]
    fn sum_of_squares_100() {
        assert_eq!(338_350, squares::sum_of_squares(100));
    }


    #[test]
    #[ignore]
    fn difference_1() {
        assert_eq!(0, squares::difference(1));
    }


    #[test]
    #[ignore]
    fn difference_5() {
        assert_eq!(170, squares::difference(5));
    }


    #[test]
    #[ignore]
    fn difference_100() {
        assert_eq!(25_164_150, squares::difference(100));
    }
}