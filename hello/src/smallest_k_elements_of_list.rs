///
/// [Smallest K Elements Of List](https://www.codewars.com/kata/648dc767d52f512bc806bf7a)
///
fn get_k_smallest<T: Copy + Ord + PartialOrd>(arr: &mut [T], k: usize) -> Vec<T> {
    if k == 0 { return vec![]; };
    quick_find_k(arr, 0, arr.len() - 1, k - 1);
    arr[0..k].to_vec()
}

fn quick_find_k<T: Copy + Ord + PartialOrd>(arr: &mut [T], l: usize, r: usize, k: usize) {

    let mut start = l;
    let flag = arr[start];
    let mut end = r;

    while start < end {

        while start < end && arr[end] > flag {
            end -= 1;
        }

        if start < end {
            arr[start] = arr[end];
            start += 1;
        }

        while start < end && arr[start] < flag {
            start += 1;
        }

        if start < end {
            arr[end] = arr[start];
            end -= 1;
        }
    }

    arr[start] = flag;

    if start != k {
        if start < k {
            quick_find_k(arr, start + 1, r, k)
        } else {
            quick_find_k(arr, l, start - 1, k)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::get_k_smallest;

    #[test]
    fn sample_tests() {
        test_correctness_helper(&mut [1, 2, 3], 0); // []
        test_correctness_helper(&mut [1, 2, 3], 1); // [1]
        test_correctness_helper(&mut [1, 2, 3], 2); // [1,2]
        test_correctness_helper(&mut [1, 2, 3], 3); // [1,2,3]

        test_correctness_helper(&mut [3, 2, 1], 1); // [1]
        test_correctness_helper(&mut [3, 2, 1], 2); // [1,2]
        test_correctness_helper(&mut [3, 2, 1], 3); // [1,2,3]

        test_correctness_helper(&mut [1, 1, 1], 3); // [1,1,1]
        test_correctness_helper(&mut [1, 1, 1], 2); // [1,1]

        test_correctness_helper(&mut [1, -5, 1], 2); //[-5,1]
        test_correctness_helper(&mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2); // [1,2]
        test_correctness_helper(&mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 9); // [1,2,3,4,5,6,7,8,9]
    }

    fn test_correctness_helper(arr: &mut [i64], k: usize) {
        let mut arr_sorted: Vec<i64> = arr.to_vec();
        arr_sorted.sort_unstable();
        // this is the sorted arr, so the first K elements here are also the smallest
        let expected: Vec<i64> = arr_sorted.iter().take(k).copied().collect();
        let ans = get_k_smallest(arr, k);
        let mut ans_sorted = ans.clone();
        ans_sorted.sort();
        // I also sort your answer, because you can give the answers in whatever order you want
        println!("sorted arr: {arr_sorted:?}, your answer: {ans:?}\nyour answer after sorting: {ans_sorted:?}, expected answer: {expected:?}\n");
        assert_eq!(ans_sorted, expected);
    }
}