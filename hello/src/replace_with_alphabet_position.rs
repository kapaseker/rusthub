fn alphabet_position(text: &str) -> String {
    let mut nums_vec = Vec::new();

    for x in text.chars() {
        let code = x as u8;
        if code >= 97 && code <= 122 {
            nums_vec.push(format!("{}", code - 96));
        }

        if (code >= 65 && code <= 90) {
            nums_vec.push(format!("{}", code - 64));
        }
    }

    nums_vec.join(" ")
}