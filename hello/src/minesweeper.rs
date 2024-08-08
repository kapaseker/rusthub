use num::traits::SaturatingSub;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.is_empty() { return vec![]; }
    let mut mines: Vec<Vec<char>> = minefield.iter().map(|s| (**s).chars().collect::<Vec<char>>()).collect();
    let rows = mines.len();
    let cols = mines[0].len();
    let def = ' ';

    (0..rows).for_each(|r| {
        (0..cols).for_each(|c| {
            if mines[r][c] == ' ' {
                let mut sum: u8 = 0;
                (r.saturating_sub(1)..=(r + 1)).for_each(|mine_row| {
                    if let Some(row) = mines.get(mine_row) {
                        (c.saturating_sub(1)..=(c + 1)).for_each(|mine_col| {
                            if *(row.get(mine_col).unwrap_or(&def)) == '*' {
                                sum += 1;
                            }
                        })
                    }
                });

                if sum != 0 {
                    mines[r][c] = char::from('0' as u8 + sum);
                }
            }
        });
    });

    mines.iter().map(|chs| chs.iter().collect::<String>()).collect()
}

#[cfg(test)]
mod test {
    use crate::minesweeper::annotate;

    #[test]
    fn case_1() {
        let vec = annotate(
            &[
                " * * ",
                "  *  ",
                "  *  ",
                "     ",
            ]
        );
        println!("{:?}", vec);
    }
}