fn alphabet_position(text: &str) -> String {
    let mut nums_vec = Vec::new();

    text.chars().map(|x| x.to_ascii_lowercase()).for_each(|x| {
        let code = x as u8;
        if code >= 97 && code <= 122 {
            nums_vec.push(code - 96);
        }
    });

    nums_vec
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")

    // for x in text.chars() {
    //     let code = x as u8;
    //     if code >= 97 && code <= 122 {
    //         nums_vec.push(format!("{}", code - 96));
    //     }
    //
    //     if code >= 65 && code <= 90 {
    //         nums_vec.push(format!("{}", code - 64));
    //     }
    // }
    //
    // nums_vec.join(" ")
}

mod test {
    use super::*;
    #[test]
    fn testCommon() {
        assert_eq!(alphabet_position("The sunset sets at twelve o' clock."),"20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11")
    }
}
