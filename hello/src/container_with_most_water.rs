
///
/// [盛最多水的容器](https://leetcode.cn/problems/container-with-most-water/description)
///
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut l = 0;
    let mut r = height.len() - 1;
    let mut max = 0;

    while l < r {
        if height[l] < height[r] {
            max = max.max(height[l] * (r - l) as i32);
            l += 1;
        } else {
            max = max.max(height[r] * (r - l) as i32);
            r -= 1;
        }
    }

    max
}


#[cfg(test)]
mod test {
    use crate::container_with_most_water::max_area;

    #[test]
    fn case_1() {
        assert_eq!(49, max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]))
    }
}