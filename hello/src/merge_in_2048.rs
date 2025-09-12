fn merge(line: &[u8]) -> Vec<u8> {
    // 1. 创建一个只包含非零元素的新向量
    let non_zeros = line.iter().filter(|&&x| x != 0).cloned().collect::<Vec<u8>>();

    // 2. 遍历非零元素列表并尝试合并相邻的相同元素
    let mut merged = Vec::new();
    let mut skip_next = false;
    for i in 0..non_zeros.len() {
        if skip_next {
            // 如果前一个元素已经被合并，则跳过当前元素
            skip_next = false;
            continue;
        }
        if i + 1 < non_zeros.len() && non_zeros[i] == non_zeros[i + 1] {
            // 当前元素与下一个元素相同，则合并它们
            merged.push(non_zeros[i] * 2);
            skip_next = true; // 标记下一次循环应该跳过
        } else {
            // 当前元素没有可以合并的对象
            merged.push(non_zeros[i]);
        }
    }

    // 3. 在末尾添加足够的零以保持数组长度不变
    while merged.len() < line.len() {
        merged.push(0);
    }

    merged
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_examples() {
        assert_eq!(merge(&[2, 0, 2, 2]), vec![4, 2, 0, 0]);
        assert_eq!(merge(&[4, 4, 8, 16]), vec![8, 8, 16, 0]);
        assert_eq!(merge(&[8, 8, 16, 0]), vec![16, 16, 0, 0]);
        assert_eq!(merge(&[16, 16, 0, 0]), vec![32, 0, 0, 0]);
    }

    #[test]
    fn test_no_merge() {
        assert_eq!(merge(&[2, 4, 8, 16]), vec![2, 4, 8, 16]);
        assert_eq!(merge(&[1, 2, 3, 4]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_all_zeros() {
        assert_eq!(merge(&[0, 0, 0, 0]), vec![0, 0, 0, 0]);
        assert_eq!(merge(&[0]), vec![0]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(merge(&[2]), vec![2]);
        assert_eq!(merge(&[0]), vec![0]);
    }

    #[test]
    fn test_complex_merges() {
        assert_eq!(merge(&[2, 2, 2, 2]), vec![4, 4, 0, 0]);
        assert_eq!(merge(&[2, 2, 4, 4]), vec![4, 8, 0, 0]);
        assert_eq!(merge(&[4, 2, 2, 4]), vec![4, 4, 4, 0]);
        assert_eq!(merge(&[2, 4, 4, 2]), vec![2, 8, 2, 0]);
    }

    #[test]
    fn test_different_lengths() {
        assert_eq!(merge(&[2, 2]), vec![4, 0]);
        assert_eq!(merge(&[2, 2, 2]), vec![4, 2, 0]);
        assert_eq!(merge(&[1, 1, 2, 2, 3, 3]), vec![2, 4, 6, 0, 0, 0]);
    }

    #[test]
    fn test_empty_array() {
        assert_eq!(merge(&[]), Vec::<u8>::new());
    }

    #[test]
    fn test_only_zeros() {
        assert_eq!(merge(&[0, 0, 0]), vec![0, 0, 0]);
    }

    #[test]
    fn test_no_adjacent_merges() {
        assert_eq!(merge(&[2, 0, 4, 0, 2]), vec![2, 4, 2, 0, 0]);
    }
}