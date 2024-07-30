use std::collections::HashMap;

/// [Find the odd int](https://www.codewars.com/kata/54da5a58ea159efa38000836/rust)
/// [有个位运算的算法非常厉害](https://www.codewars.com/kata/54da5a58ea159efa38000836/solutions/rust)
fn find_odd(arr: &[i32]) -> i32 {

    let map:HashMap<i32,u32> = arr.iter().fold(HashMap::new(), |mut map, value| {
        *map.entry(*value).or_default() += 1;
        map
    });

    for (x,c) in map {
        if c & 1 == 1 {
            return x
        }
    }

    return 0
}

#[cfg(test)]
mod tests {
    use super::find_odd;

    #[test]
    fn basic_tests() {
        assert_eq!(find_odd(&vec![20,1,-1,2,-2,3,3,5,5,1,2,4,20,4,-1,-2,5]), 5);
        assert_eq!(find_odd(&vec![1,1,2,-2,5,2,4,4,-1,-2,5]), -1);
        assert_eq!(find_odd(&vec![20,1,1,2,2,3,3,5,5,4,20,4,5]), 5);
        assert_eq!(find_odd(&vec![10]), 10);
        assert_eq!(find_odd(&vec![1,1,1,1,1,1,10,1,1,1,1]), 10);
        assert_eq!(find_odd(&vec![5,4,3,2,1,5,4,3,2,10,10]), 1);
    }
}