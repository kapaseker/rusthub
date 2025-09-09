pub fn answer(command: &str) -> Option<i32> {
    // 检查命令是否以"What is "开头
    if !command.starts_with("What is ") {
        return None;
    }

    // 提取表达式部分并移除末尾的问号
    let expr = command
        .strip_prefix("What is ")?
        .strip_suffix("?")
        .unwrap_or("");

    // 解析表达式
    parse_expression(expr)
}

fn parse_expression(expr: &str) -> Option<i32> {
    if expr.is_empty() {
        return None;
    }

    let tokens: Vec<&str> = expr.split_whitespace().collect();
    let mut i = 0;

    // 解析第一个数字
    let mut result = tokens.get(i).and_then(|s| s.parse::<i32>().ok())?;
    i += 1;

    // 处理操作符和后续数字
    while i < tokens.len() {
        match *tokens.get(i)? {
            "plus" => {
                i += 1;
                let next_number = tokens.get(i).and_then(|s| s.parse::<i32>().ok())?;
                result = result.checked_add(next_number)?;
                i += 1;
            }
            "minus" => {
                i += 1;
                let next_number = tokens.get(i).and_then(|s| s.parse::<i32>().ok())?;
                result = result.checked_sub(next_number)?;
                i += 1;
            }
            "multiplied" => {
                // 检查是否有"by"
                if i + 1 < tokens.len() && tokens[i + 1] == "by" {
                    i += 2;
                    let next_number = tokens.get(i).and_then(|s| s.parse::<i32>().ok())?;
                    result = result.checked_mul(next_number)?;
                    i += 1;
                } else {
                    return None;
                }
            }
            "divided" => {
                // 检查是否有"by"
                if i + 1 < tokens.len() && tokens[i + 1] == "by" {
                    i += 2;
                    let next_number = tokens.get(i).and_then(|s| s.parse::<i32>().ok())?;
                    result = result.checked_div(next_number)?;
                    i += 1;
                } else {
                    return None;
                }
            }
            // 处理指数运算: "raised to the Nth power"
            "raised" => {
                // 检查是否是完整的 "raised to the Nth power" 格式
                if i + 4 < tokens.len()
                    && tokens[i + 1] == "to"
                    && tokens[i + 2] == "the"
                    && tokens[i + 4] == "power" {
                    // 提取指数 (Nth 中的 N)
                    let exp_str = tokens[i + 3];
                    let exp = parse_ordinal_number(exp_str)?;
                    result = power(result, exp)?;
                    i += 5; // 跳过 "raised" "to" "the" Nth "power"
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    Some(result)
}

// 解析序数词数字 (如 5th, 2nd 等)
fn parse_ordinal_number(ord: &str) -> Option<i32> {
    // 需要处理特殊情况: 1st, 2nd, 3rd, 和其他以 th 结尾的
    if ord.len() < 3 {
        return None;
    }

    let suffixes = ["st", "nd", "rd", "th"];
    for &suffix in &suffixes {
        if let Some(stripped) = ord.strip_suffix(suffix) {
            return stripped.parse::<i32>().ok();
        }
    }
    None
}

// 计算 a 的 b 次方，使用 checked arithmetic 防止溢出
fn power(base: i32, exp: i32) -> Option<i32> {
    if exp < 0 {
        return None;
    }

    let exp = exp as u32;
    (0..exp).try_fold(1i32, |acc, _| acc.checked_mul(base))
}



mod test {
    use crate::wordy::answer;

    #[test]

    fn just_a_number() {
        let input = "What is 5?";

        let output = answer(input);

        let expected = Some(5);

        assert_eq!(output, expected);
    }

    #[test]
    fn addition() {
        let input = "What is 1 plus 1?";

        let output = answer(input);

        let expected = Some(2);

        assert_eq!(output, expected);
    }

    #[test]
    fn more_addition() {
        let input = "What is 53 plus 2?";

        let output = answer(input);

        let expected = Some(55);

        assert_eq!(output, expected);
    }

    #[test]
    fn addition_with_negative_numbers() {
        let input = "What is -1 plus -10?";

        let output = answer(input);

        let expected = Some(-11);

        assert_eq!(output, expected);
    }

    #[test]
    fn large_addition() {
        let input = "What is 123 plus 45678?";

        let output = answer(input);

        let expected = Some(45801);

        assert_eq!(output, expected);
    }

    #[test]
    fn subtraction() {
        let input = "What is 4 minus -12?";

        let output = answer(input);

        let expected = Some(16);

        assert_eq!(output, expected);
    }

    #[test]
    fn multiplication() {
        let input = "What is -3 multiplied by 25?";

        let output = answer(input);

        let expected = Some(-75);

        assert_eq!(output, expected);
    }

    #[test]
    fn division() {
        let input = "What is 33 divided by -3?";

        let output = answer(input);

        let expected = Some(-11);

        assert_eq!(output, expected);
    }

    #[test]
    fn multiple_additions() {
        let input = "What is 1 plus 1 plus 1?";

        let output = answer(input);

        let expected = Some(3);

        assert_eq!(output, expected);
    }

    #[test]
    fn addition_and_subtraction() {
        let input = "What is 1 plus 5 minus -2?";

        let output = answer(input);

        let expected = Some(8);

        assert_eq!(output, expected);
    }

    #[test]
    fn multiple_subtraction() {
        let input = "What is 20 minus 4 minus 13?";

        let output = answer(input);

        let expected = Some(3);

        assert_eq!(output, expected);
    }

    #[test]
    fn subtraction_then_addition() {
        let input = "What is 17 minus 6 plus 3?";

        let output = answer(input);

        let expected = Some(14);

        assert_eq!(output, expected);
    }

    #[test]
    fn multiple_multiplication() {
        let input = "What is 2 multiplied by -2 multiplied by 3?";

        let output = answer(input);

        let expected = Some(-12);

        assert_eq!(output, expected);
    }

    #[test]
    fn addition_and_multiplication() {
        let input = "What is -3 plus 7 multiplied by -2?";

        let output = answer(input);

        let expected = Some(-8);

        assert_eq!(output, expected);
    }

    #[test]
    fn multiple_division() {
        let input = "What is -12 divided by 2 divided by -3?";

        let output = answer(input);

        let expected = Some(2);

        assert_eq!(output, expected);
    }

    #[test]
    fn unknown_operation() {
        let input = "What is 52 cubed?";

        let output = answer(input);

        let expected = None;

        assert_eq!(output, expected);
    }

    #[test]
    fn non_math_question() {
        let input = "Who is the President of the United States?";

        let output = answer(input);

        let expected = None;

        assert_eq!(output, expected);
    }

    #[test]
    fn reject_problem_missing_an_operand() {
        let input = "What is 1 plus?";

        let output = answer(input);

        let expected = None;

        assert_eq!(output, expected);
    }

    #[test]
    fn reject_problem_with_no_operands_or_operators() {
        let input = "What is?";

        let output = answer(input);

        let expected = None;

        assert_eq!(output, expected);
    }

    #[test]
    fn reject_two_operations_in_a_row() {
        let input = "What is 1 plus plus 2?";

        let output = answer(input);

        let expected = None;

        assert_eq!(output, expected);
    }

    #[test]
    fn reject_two_numbers_in_a_row() {
        let input = "What is 1 plus 2 1?";

        let output = answer(input);

        let expected = None;

        assert_eq!(output, expected);
    }

    #[test]
    fn reject_postfix_notation() {
        let input = "What is 1 2 plus?";

        let output = answer(input);

        let expected = None;

        assert_eq!(output, expected);
    }

    #[test]
    fn reject_prefix_notation() {
        let input = "What is plus 1 2?";

        let output = answer(input);

        let expected = None;

        assert_eq!(output, expected);
    }

    #[test]fn exponential() {
        let input = "What is 2 raised to the 5th power?";

        let output = answer(input);

        let expected = Some(32);

        assert_eq!(output, expected);
    }

    #[test]fn addition_and_exponential() {
        let input = "What is 1 plus 2 raised to the 2nd power?";

        let output = answer(input);

        let expected = Some(9);

        assert_eq!(output, expected);
    }
}
