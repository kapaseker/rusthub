use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number, input your guess:");

    let secret = rand::thread_rng().gen_range(1..=100);

    // println!("secret is : {secret}");

    let mut try_time = 1;

    loop {
        let mut guess = String::new();

        // 会输入回车
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let mut guess_number = 0;

        match guess.trim().parse::<u32>() {
            Ok(num) => guess_number = num,
            Err(_) => {
                if guess.trim().starts_with("q") {
                    println!("Quit game.");
                    break;
                }
                continue;
            }
        }

        match guess_number.cmp(&secret) {
            Ordering::Less => { println!("Too small.") }
            Ordering::Equal => {
                println!("Great, Right, You win !");
                break;
            }
            Ordering::Greater => { println!("Too big.") }
        }

        try_time += 1;
    }

    println!("You've try {try_time} times");
}
