use crate::prime::is_prime;

pub fn nth(n: u32) -> u32 {
    let mut m = 2;
    let mut i = 0;

    while i <= n {
        if is_prime(m) {
            i += 1;
        }
        m += 1;
    }
    m - 1
}

mod test {
    use crate::nth_prime::nth;

    #[test]
    fn first_prime() {
        let output = nth(0);
        let expected = 2;
        assert_eq!(output, expected);
    }


    #[test]
    #[ignore]
    fn second_prime() {
        let output = nth(1);


        let expected = 3;


        assert_eq!(output, expected);
    }


    #[test]
    #[ignore]
    fn sixth_prime() {
        let output = nth(5);


        let expected = 13;


        assert_eq!(output, expected);
    }


    #[test]
    #[ignore]
    fn big_prime() {
        let output = nth(10000);


        let expected = 104743;


        assert_eq!(output, expected);
    }
}