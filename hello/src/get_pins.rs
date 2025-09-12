// 其实这个就是求一个笛卡尔积
fn get_pins(observed: &str) -> Vec<String> {
    // 定义每个数字的相邻数字（包括自己）
    let adjacent = [
        vec!['0', '8'],                // 0 的相邻数字：0, 8
        vec!['1', '2', '4'],           // 1 的相邻数字：1, 2, 4
        vec!['2', '1', '3', '5'],      // 2 的相邻数字：2, 1, 3, 5
        vec!['3', '2', '6'],           // 3 的相邻数字：3, 2, 6
        vec!['4', '1', '5', '7'],      // 4 的相邻数字：4, 1, 5, 7
        vec!['5', '2', '4', '6', '8'], // 5 的相邻数字：5, 2, 4, 6, 8
        vec!['6', '3', '5', '9'],      // 6 的相邻数字：6, 3, 5, 9
        vec!['7', '4', '8'],           // 7 的相邻数字：7, 4, 8
        vec!['8', '5', '7', '9', '0'], // 8 的相邻数字：8, 5, 7, 9, 0
        vec!['9', '6', '8'],           // 9 的相邻数字：9, 6, 8
    ];

    // 将观察到的字符串转换为字符向量
    let digits: Vec<char> = observed.chars().collect();
    let mut result = Vec::new();

    // 使用递归生成所有可能的组合
    fn generate_combinations(
        current: String,
        index: usize,
        digits: &[char],
        adjacent: &[Vec<char>; 10],
        result: &mut Vec<String>,
    ) {
        // 如果已经处理完所有位数，将当前组合加入结果
        if index == digits.len() {
            result.push(current);
            return;
        }

        // 获取当前位数字对应的字符
        let digit_char = digits[index];
        // 转换为数字索引（'0' -> 0, '1' -> 1, ...）
        let digit_index = digit_char.to_digit(10).unwrap() as usize;

        // 遍历当前数字的所有相邻数字（包括自己）
        for &adj_digit in &adjacent[digit_index] {
            let mut new_current = current.clone();
            new_current.push(adj_digit);
            // 递归处理下一位
            generate_combinations(new_current, index + 1, digits, adjacent, result);
        }
    }

    // 开始生成组合
    generate_combinations(String::new(), 0, &digits, &adjacent, &mut result);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        // 测试单个数字
        let result = get_pins("8");
        assert_eq!(result.len(), 5);
        assert!(result.contains(&"0".to_string()));
        assert!(result.contains(&"5".to_string()));
        assert!(result.contains(&"7".to_string()));
        assert!(result.contains(&"8".to_string()));
        assert!(result.contains(&"9".to_string()));

        // 测试两个数字
        let result = get_pins("11");
        assert_eq!(result.len(), 9);
        assert!(result.contains(&"11".to_string()));
        assert!(result.contains(&"12".to_string()));
        assert!(result.contains(&"14".to_string()));
        assert!(result.contains(&"21".to_string()));
        assert!(result.contains(&"22".to_string()));
        assert!(result.contains(&"24".to_string()));
        assert!(result.contains(&"41".to_string()));
        assert!(result.contains(&"42".to_string()));
        assert!(result.contains(&"44".to_string()));

        // 测试题目中的例子 1357
        let result = get_pins("1357");
        println!("1357的所有可能组合: {:?}", result);
        assert!(result.contains(&"1357".to_string()));
        assert!(result.contains(&"2357".to_string()));
        assert!(result.contains(&"4357".to_string()));
        assert!(result.contains(&"1257".to_string()));
        assert!(result.contains(&"1657".to_string()));
        assert!(result.contains(&"1327".to_string()));
        assert!(result.contains(&"1367".to_string()));
        assert!(result.contains(&"1354".to_string()));
        assert!(result.contains(&"1358".to_string()));
        // 应该有 3 * 3 * 5 * 3 = 90 种组合
        assert_eq!(result.len(), 135);
    }

    #[test]
    fn test_edge_cases() {
        // 测试空字符串
        let result = get_pins("");
        assert_eq!(result, vec!["".to_string()]);

        // 测试包含0的情况
        let result = get_pins("0");
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"0".to_string()));
        assert!(result.contains(&"8".to_string()));

        // 测试长PIN码
        let result = get_pins("123");
        assert_eq!(result.len(), 3 * 4 * 3); // 1有3种可能，2有4种，3有3种
    }
}

// 也可以提供一个非递归版本（使用迭代方法）
fn get_pins_iterative(observed: &str) -> Vec<String> {
    if observed.is_empty() {
        return vec!["".to_string()];
    }

    let adjacent = [
        vec!['0', '8'],
        vec!['1', '2', '4'],
        vec!['2', '1', '3', '5'],
        vec!['3', '2', '6'],
        vec!['4', '1', '5', '7'],
        vec!['5', '2', '4', '6', '8'],
        vec!['6', '3', '5', '9'],
        vec!['7', '4', '8'],
        vec!['8', '5', '7', '9', '0'],
        vec!['9', '6', '8'],
    ];

    let mut combinations = vec!["".to_string()];

    for digit_char in observed.chars() {
        let digit_index = digit_char.to_digit(10).unwrap() as usize;
        let mut new_combinations = Vec::new();

        for prefix in &combinations {
            for &adj_digit in &adjacent[digit_index] {
                let mut new_combo = prefix.clone();
                new_combo.push(adj_digit);
                new_combinations.push(new_combo);
            }
        }

        combinations = new_combinations;
    }

    combinations
}

#[cfg(test)]
mod iterative_tests {
    use super::*;

    #[test]
    fn test_iterative_version() {
        let result1 = get_pins("1357");
        let result2 = get_pins_iterative("1357");

        assert_eq!(result1.len(), result2.len());
        for combo in &result1 {
            assert!(result2.contains(combo));
        }
        for combo in &result2 {
            assert!(result1.contains(combo));
        }
    }
}

// 主函数示例
fn main() {
    println!("测试PIN码 '1357' 的所有可能组合:");
    let pins = get_pins("1357");
    println!("总共 {} 种可能组合:", pins.len());

    // 打印前10个和后10个组合（如果总数超过20个）
    if pins.len() <= 20 {
        for pin in &pins {
            println!("{}", pin);
        }
    } else {
        println!("前10个:");
        for pin in pins.iter().take(10) {
            println!("{}", pin);
        }
        println!("...");
        println!("后10个:");
        for pin in pins.iter().skip(pins.len() - 10) {
            println!("{}", pin);
        }
    }
}
