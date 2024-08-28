pub struct PascalsTriangle(u32);


impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (1..=self.0).map(|v| {
            let mut row_vec = Vec::with_capacity(v as usize);
            row_vec.push(1);
            (1..v).for_each(|r| {
                row_vec.push(row_vec[r as usize - 1] * (v - r) / r); //二项式递推公式：当前项等于前一项 * (n - (k - 1) / k)，n = v - 1，所以 (v - r) / r。
            });
            row_vec
        }).collect()
    }
}

#[cfg(test)]
mod test {
    use crate::pascals_triangle::PascalsTriangle;
    use crate::util::cn_k;

    #[test]
    fn case_n_k() {
        assert_eq!(6, cn_k(4, 2));

        assert_eq!(cn_k(6, 4), cn_k(6, 2));

        assert_eq!(15, cn_k(6, 2));

        assert_eq!(126, cn_k(9, 4));
    }

    #[test]

    fn zero_rows() {
        let pt = PascalsTriangle::new(0);


        let expected: Vec<Vec<u32>> = vec![];


        assert_eq!(pt.rows(), expected);
    }


    #[test]

    fn single_row() {
        let pt = PascalsTriangle::new(1);


        let expected: Vec<Vec<u32>> = vec![vec![1]];


        assert_eq!(pt.rows(), expected);
    }


    #[test]

    fn two_rows() {
        let pt = PascalsTriangle::new(2);


        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1]];


        assert_eq!(pt.rows(), expected);
    }


    #[test]

    fn three_rows() {
        let pt = PascalsTriangle::new(3);


        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1]];


        assert_eq!(pt.rows(), expected);
    }


    #[test]

    fn four_rows() {
        let pt = PascalsTriangle::new(4);


        let expected: Vec<Vec<u32>> = vec![vec![1], vec![1, 1], vec![1, 2, 1], vec![1, 3, 3, 1]];


        assert_eq!(pt.rows(), expected);
    }


    #[test]

    fn five_rows() {
        let pt = PascalsTriangle::new(5);


        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];


        assert_eq!(pt.rows(), expected);
    }


    #[test]

    fn six_rows() {
        let pt = PascalsTriangle::new(6);


        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
        ];


        assert_eq!(pt.rows(), expected);
    }


    #[test]

    fn ten_rows() {
        let pt = PascalsTriangle::new(10);


        let expected: Vec<Vec<u32>> = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1],
            vec![1, 7, 21, 35, 35, 21, 7, 1],
            vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
            vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
        ];


        assert_eq!(pt.rows(), expected);
    }
}

