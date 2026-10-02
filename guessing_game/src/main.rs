use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess numbers!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("input your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to get your number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("that's not a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "too small".red()),
            Ordering::Greater => println!("{}", "too big".red()),
            Ordering::Equal => {
                println!("{}", "you win".green());
                break;
            }
        }
    }
}
