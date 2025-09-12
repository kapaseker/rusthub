fn rot13(text: &str) -> String {
    text.chars()
        .map(|c| {
            match c {
                'a'..='m' | 'A'..='M' => ((c as u8) + 13) as char,
                'n'..='z' | 'N'..='Z' => ((c as u8) - 13) as char,
                _ => c, // 非字母字符保持不变
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot13_examples() {
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
        assert_eq!(rot13("Hello, World!"), "Uryyb, Jbeyq!");
        assert_eq!(rot13("123"), "123");
        assert_eq!(rot13("ROT13 example."), "EBG13 rknzcyr.");
        assert_eq!(rot13(""), "");
    }

    #[test]
    fn test_rot13_roundtrip() {
        let original = "The Quick Brown Fox Jumps Over The Lazy Dog!";
        let encrypted = rot13(original);
        let decrypted = rot13(&encrypted);
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(rot13("abcdefghijklm"), "nopqrstuvwxyz");
        assert_eq!(rot13("nopqrstuvwxyz"), "abcdefghijklm");
        assert_eq!(rot13("ABCDEFGHIJKLM"), "NOPQRSTUVWXYZ");
        assert_eq!(rot13("NOPQRSTUVWXYZ"), "ABCDEFGHIJKLM");
        assert_eq!(rot13("a1B2c3D4"), "n1O2p3Q4");
    }
}

fn main() {
    // 示例用法
    let original = "Hello, World!";
    let encrypted = rot13(original);
    let decrypted = rot13(&encrypted);

    println!("原文: {}", original);
    println!("加密: {}", encrypted);
    println!("解密: {}", decrypted);

    // 验证 ROT13 是自己的逆函数
    assert_eq!(original, decrypted);
    println!("验证通过: ROT13 是自己的逆函数");
}