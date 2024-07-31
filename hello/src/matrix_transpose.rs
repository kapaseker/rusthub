fn transpose(matrix: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let row = matrix.len();
    let col = matrix[0].len();

    let mut transpose:Vec<Vec<u8>> = Vec::with_capacity(col);

    for c in 0..col {
        let mut rows = Vec::with_capacity(row);
        for r in 0..row {
            rows.push(matrix[r][c]);
        }
        transpose.push(rows);
    }

    transpose
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::transpose;

    #[test]
    fn sample_tests() {
        assert_eq!(transpose(&[vec![1]]), vec![vec![1]]);
        assert_eq!(transpose(&[vec![1, 2, 3]]), vec![vec![1], vec![2], vec![3]]);
        assert_eq!(transpose(&[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]);
        assert_eq!(transpose(&[vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1], vec![0, 1, 0], vec![1, 0, 0]]), vec![vec![1, 0, 0, 0, 1], vec![0, 1, 0, 1, 0], vec![0, 0, 1, 0, 0]]);
    }
}