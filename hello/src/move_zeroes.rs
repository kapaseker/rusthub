pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut l_zero = 0;
    let mut r_num = 0;
    let len = nums.len();

    loop {
        while l_zero < len && nums[l_zero] != 0 {
            l_zero += 1;
        }
        r_num = (l_zero + 1).max(r_num + 1);
        while r_num < len && nums[r_num] == 0 {
            r_num += 1;
        }
        if l_zero < len && r_num < len {
            nums.swap(l_zero, r_num);
            l_zero += 1;
        } else {
            break;
        }
    }
}


#[cfg(test)]
mod test {
    use crate::move_zeroes::move_zeroes;

    #[test]
    fn case_1() {
        let mut a = vec![0, 1, 0, 3, 12];
        move_zeroes(&mut a);
        assert_eq!(a, vec![1, 3, 12, 0, 0]);
    }

    #[test]
    fn case_2() {
        let mut a = vec![0];
        move_zeroes(&mut a);
        assert_eq!(a, vec![0]);
    }

    #[test]
    fn case_3() {
        let mut a = vec![1, 2, 0];
        move_zeroes(&mut a);
        assert_eq!(a, vec![1, 2, 0]);
    }
}