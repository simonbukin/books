use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess:");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

// double quoted strings, not single
// :: are functions ON types (neat)
// everything println! string is a template string, but only variable can be directly interpolated
// "{wow}" but not "{wow + 2}"
// cargo doc --open is a super neat feature. Wonder if that could be ported to node?
// shadowing is cool. no need for extra throwaway variables to think of names for
