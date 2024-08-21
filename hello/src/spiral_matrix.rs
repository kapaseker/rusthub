pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0u32; size as usize]; size as usize];
    if size == 0 { return matrix; };
    
    let count = (size * size) as usize;
    let (mut l, mut r, mut t, mut b, mut row, mut col) = (0, (size - 1) as usize, 0, (size - 1) as usize, 0, 0);
    let mut d = 0;

    let mut spiral = 1;

    loop {
        matrix[row][col] = spiral;

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

        spiral += 1;
        if spiral > count as u32 {
            break;
        }
    }

    matrix
}

mod test {
    use crate::spiral_matrix::spiral_matrix;

    #[test]

    fn empty_spiral() {
        let input = 0;


        let output = spiral_matrix(input);


        let expected: [[u32; 0]; 0] = [];


        assert_eq!(output, expected);
    }


    #[test]
    

    fn trivial_spiral() {
        let input = 1;


        let output = spiral_matrix(input);


        let expected: [[u32; 1]; 1] = [[1]];


        assert_eq!(output, expected);
    }


    #[test]
    

    fn spiral_of_size_2() {
        let input = 2;


        let output = spiral_matrix(input);


        let expected: [[u32; 2]; 2] = [[1, 2], [4, 3]];


        assert_eq!(output, expected);
    }


    #[test]
    

    fn spiral_of_size_3() {
        let input = 3;


        let output = spiral_matrix(input);


        let expected: [[u32; 3]; 3] = [[1, 2, 3], [8, 9, 4], [7, 6, 5]];


        assert_eq!(output, expected);
    }


    #[test]
    

    fn spiral_of_size_4() {
        let input = 4;


        let output = spiral_matrix(input);


        let expected: [[u32; 4]; 4] = [
            [1, 2, 3, 4],
            [12, 13, 14, 5],
            [11, 16, 15, 6],
            [10, 9, 8, 7],
        ];


        assert_eq!(output, expected);
    }


    #[test]
    

    fn spiral_of_size_5() {
        let input = 5;


        let output = spiral_matrix(input);


        let expected: [[u32; 5]; 5] = [
            [1, 2, 3, 4, 5],
            [16, 17, 18, 19, 6],
            [15, 24, 25, 20, 7],
            [14, 23, 22, 21, 8],
            [13, 12, 11, 10, 9],
        ];


        assert_eq!(output, expected);
    }
}