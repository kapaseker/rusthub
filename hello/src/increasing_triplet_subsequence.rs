
///
/// [递增的三元子序列](https://leetcode.cn/problems/increasing-triplet-subsequence/)
///
pub fn increasing_triplet(nums: Vec<i32>) -> bool {
    if nums.len() < 3 { return false; }

    let mut a = nums[0];
    let mut b = i32::MAX;

    for i in 1..nums.len() {
        let c = nums[i];
        if c > b {
            return true;
        } else if c > a {
            b = c;
        } else {
            a = c;
        }
    }

    return false
}

#[cfg(test)]
mod test {
    use crate::increasing_triplet_subsequence::increasing_triplet;

    #[test]
    fn case_1() {
        assert!(increasing_triplet(vec![1,2,3,4,5]))
    }

    #[test]
    fn case_2() {
        assert!(!increasing_triplet(vec![1,2,3,4,5].into_iter().rev().collect()));
    }

    #[test]
    fn case_3() {
        assert!(increasing_triplet(vec![2,1,5,0,4,6]))
    }
}