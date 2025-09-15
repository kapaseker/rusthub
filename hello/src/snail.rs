fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let n = if matrix.is_empty() || matrix[0].is_empty() { 0 } else { matrix.len() };
    if n == 0 { return vec![]; }
    let count = n * n;
    let (mut l, mut r, mut t, mut b, mut row, mut col) = (0, n - 1, 0, n - 1, 0, 0);
    let mut d = 0;
    let mut s = 0;
    let mut array = Vec::with_capacity(count);

    loop {

        array.push(matrix[row][col]);

        match d {
            0 => {
                if col == r {
                    d = 1;
                    t += 1;
                    row += 1;
                } else {
                    col += 1;
                }
            }
            1 => {
                if row == b {
                    d = 2;
                    r -= 1;
                    col -= 1;
                } else {
                    row += 1;
                }
            }
            2 => {
               if col == l {
                    d = 3;
                    b -= 1;
                    row -= 1;
                } else {
                    col -= 1;
                }
            }
            _ => {
                if row == t {
                    d = 0;
                    l += 1;
                    col += 1;
                } else {
                    row -= 1;
                }
            }
        }


        s += 1;
        if s == n * n {
            break;
        }
    }


    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test1() {
        let square = &[
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test2() {
        let square = &[
            vec![1, 2, 3],
            vec![8, 9, 4],
            vec![7, 6, 5],
        ];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(snail(square), expected);
    }

    #[test]
    fn sample_test3() {
        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected: Vec<i32> = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");
    }

    #[test]
    fn sample_test4() {
        let square = &[vec![1]];
        let expected = vec![1];
        assert_eq!(snail(square), expected);
    }
}
