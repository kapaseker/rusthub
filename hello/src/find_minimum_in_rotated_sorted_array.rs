///
/// [寻找旋转排序数组中的最小值](https://leetcode.cn/problems/find-minimum-in-rotated-sorted-array/)
///
pub fn find_min(nums: Vec<i32>) -> i32 {

    let mut a = 0;
    let mut b = nums.len() - 1;

    if b == 0 || nums[a] < nums[b] {
        return nums[a];
    }

    while b - a != 1 {
        let mid = (b + a) / 2;
        if nums[mid] > nums[b] {
            a = mid;
        } else {
            b = mid;
        }
    }

    nums[a].min(nums[b])
}

#[cfg(test)]
mod test {
    use super::find_min;

    #[test]
    fn case_1() {
        assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0)
    }

    #[test]
    fn case_2() {
        assert_eq!(find_min(vec![4, 5, 6, 7, 1, 2]), 1)
    }

    #[test]
    fn case_3() {
        assert_eq!(find_min(vec![1, 2]), 1)
    }

    #[test]
    fn case_4() {
        assert_eq!(find_min(vec![2, 1]), 1)
    }

    #[test]
    fn case_5() {
        assert_eq!(find_min(vec![3, 1, 2]), 1)
    }

    #[test]
    fn case_6() {
        assert_eq!(find_min(vec![2, 3, 1]), 1)
    }
}