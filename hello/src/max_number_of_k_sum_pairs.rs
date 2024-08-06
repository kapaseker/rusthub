use std::collections::HashMap;
pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut num_set: HashMap<i32, i32> = HashMap::new();
    let mut count = 0;

    for x in &nums {
        let rem = k - x;
        let value = num_set.get_mut(&rem);
        if value.is_some() {
            count += 1;
            let mut value = value.unwrap();
            *value -= 1;
            if *value == 0 {
                num_set.remove(&rem);
            }
        } else {
            num_set.entry(*x).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    count
}


#[cfg(test)]
mod test {
    use crate::max_number_of_k_sum_pairs::max_operations;

    #[test]
    fn case_1() {
        assert_eq!(2, max_operations(vec![1, 2, 3, 4], 5))
    }

    #[test]
    fn case_2() {
        assert_eq!(1, max_operations(vec![3, 1, 3, 4, 3], 6))
    }

    #[test]
    fn case_3() {
        assert_eq!(2, max_operations(vec![3, 3, 3, 3], 6))
    }

    #[test]
    fn case_4() {
        assert_eq!(2, max_operations(vec![3, 3, 3, 3, 3], 6))
    }

    #[test]
    fn case_5() {
        assert_eq!(2, max_operations(vec![1,1,1,1,1,2,2,], 3))
    }

    #[test]
    fn case_6() {
        assert_eq!(4, max_operations(vec![2,5,4,4,1,3,4,4,1,4,4,1,2,1,2,2,3,2,4,2], 3))
    }

    #[test]
    fn case_7() {
        assert_eq!(4, max_operations(vec![1,2,1,2,1,1,2,2], 3))
    }
}