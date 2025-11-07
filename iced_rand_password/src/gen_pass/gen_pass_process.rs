use rand::prelude::{IndexedRandom, SliceRandom};

const LOWERCASE: &str = "abcdefghijkmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHJKLMNPQRSTUVWXYZ";
const NUMBER: &str = "123456789";
// !@#$%^&*()_=+[{]}<>?
const SYMBOL: &str = "!@#$%^&*()_=+[{]}<>?";

// generate random password
pub fn process_gen_pass(
    len: u8,
    lowercase: bool,
    uppercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut rand = rand::rng();
    let mut require_chars = vec![];
    let mut password = vec![];

    let mut add_required_chars = |chars: &str| {
        if !chars.is_empty() {
            let char_vec: Vec<char> = chars.chars().collect();
            let ch = char_vec.choose(&mut rand).unwrap();
            password.push(*ch);
            require_chars.extend(char_vec);
        }
    };

    if lowercase {
        add_required_chars(LOWERCASE);
    }

    if uppercase {
        add_required_chars(UPPERCASE);
    }

    if number {
        add_required_chars(NUMBER);
    }

    if symbol {
        // !@#$%^&*()_=+[{]}<>?
        add_required_chars(SYMBOL);
    }

    if password.len() > len as usize {
        return Err(anyhow::anyhow!("Invalid password length"));
    }

    // select random char in chars
    if !require_chars.is_empty() {
        for _ in password.len() as u8..len {
            let ch = require_chars
                .choose(&mut rand)
                .expect("Failed to choose random char, chars won't be empty");
            password.push(*ch);
        }
    }

    password.shuffle(&mut rand);

    println!("Generated password: {}", password.iter().collect::<String>());

    Ok(password.iter().collect::<String>())
}
