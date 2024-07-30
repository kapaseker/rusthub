use std::fmt::format;

fn parse_bank_account(bank_account: &str) -> u64 {
    let zero = " _ \n| |\n|_|";
    let one = "   \n  |\n  |";
    let two = " _ \n _|\n|_ ";
    let three = " _ \n _|\n _|";
    let four = "   \n|_|\n  |";
    let five = " _ \n|_ \n _|";
    let six = " _ \n|_ \n|_|";
    let seven = " _ \n  |\n  |";
    let eight = " _ \n|_|\n|_|";
    let nine = " _ \n|_|\n _|";
    let nums_font = vec![zero, one, two, three, four, five, six, seven, eight, nine];
    let three_line = bank_account.split('\n').collect::<Vec<&str>>();
    let gap = 3;
    let mut sum:u64 = 0;
    for x in (0..three_line[0].len()).step_by(gap) {
        let num = format!("{}\n{}\n{}", &three_line[0][x..x + gap], &three_line[1][x..x + gap], &three_line[2][x..x + gap]);
        let num = nums_font.iter().position(|&s| s == num).unwrap();
        sum = sum * 10 + num as u64;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(123456789, parse_bank_account(concat!(
        "    _  _     _  _  _  _  _ \n",
        "  | _| _||_||_ |_   ||_||_|\n",
        "  ||_  _|  | _||_|  ||_| _|\n"
        )));
        assert_eq!(23056789, parse_bank_account(concat!(
        " _  _  _  _  _  _  _  _  _ \n",
        "| | _| _|| ||_ |_   ||_||_|\n",
        "|_||_  _||_| _||_|  ||_| _|\n"
        )));
        assert_eq!(823856989, parse_bank_account(concat!(
        " _  _  _  _  _  _  _  _  _ \n",
        "|_| _| _||_||_ |_ |_||_||_|\n",
        "|_||_  _||_| _||_| _||_| _|\n"
        )));
    }
}