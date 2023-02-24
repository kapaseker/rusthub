fn main() {
    let s = String::from("this is apple");
    let mut return_s = take_auth(s);

    println!("{return_s}");
    make_push_str(&mut return_s);
    println!("{return_s}");

    print_len(&return_s);

    println!("{}", first_word_index("hello it is me"));

    let mut  s = String::from("apple is good");
    let word = first_word_index(&s);
    println!("{}", word);
    s.clear();
}

fn take_auth(s: String) -> String {
    println!("{s}");
    s
}

fn make_push_str(s: &mut String) {
    s.push_str(" Google");
}

fn print_len(len_s: &String) {
    println!("{}", len_s.len())
}

fn first_word_index(s:&str) -> &str {

    for (i,&v) in s.as_bytes().iter().enumerate() {
        if v == b' ' {
            return &s[..i]
        }
    }


    return &s[..]
}
