use num::traits::float::FloatCore;

///
/// [子数组最大平均数 I](https://leetcode.cn/problems/maximum-average-subarray-i)
///
pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {

    let k = k as usize;
    let len = nums.len();
    let mut sum: i32 = nums[0..k].iter().sum();
    let mut max_av = sum as f64 / k as f64;

    for i in 1..=(len - k) {
        sum = sum - nums[i - 1] + nums[i + k - 1];
        max_av = max_av.max(sum as f64 / k as f64);
    }

    max_av
}

#[cfg(test)]
mod test {
    use crate::maximum_average_subarray_i::find_max_average;

    #[test]
    fn case_1() {
        assert_eq!(12.75, find_max_average(vec![1, 12, -5, -6, 50, 3], 4))
    }

    #[test]
    fn case_2() {
        assert_eq!(3.0, find_max_average(vec![4, 2, 1, 3, 3], 2))
    }
}