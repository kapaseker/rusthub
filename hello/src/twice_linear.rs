pub fn dbl_linear(n: u32) -> u32{

    if n == 0u32 { return 1u32; }

    let mut nums = vec![1u32];
    let mut gap = 1u32;
    let mut step = 0u32;

    loop {

        let end = step + gap;

        for i in step..end {
            nums.push(nums[i as usize] * 2u32 + 1u32);
        }

        for i in step..end {
            nums.push(nums[i as usize] * 3u32 + 1u32);
        }

        if nums.len() > (n as usize) {
            nums.sort();
            nums.dedup();
            if nums.len() > (n as usize) {
                println!("{:?}", nums);
                return nums[n as usize];
            }
        }

        step += gap;
        gap = gap * 2;
    }
}

mod test {
    use crate::twice_linear::dbl_linear;

    #[test]
    fn case_1() {
        assert_eq!(dbl_linear(10), 22)
    }

    #[test]
    fn case_2() {
        assert_eq!(dbl_linear(30), 91)
    }

    #[test]
    fn case_3() {
        assert_eq!(dbl_linear(50), 175)
    }
}
