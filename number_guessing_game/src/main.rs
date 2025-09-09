// Followed the tutorial from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MAX_VALID_ATTEMPTS: u8 = 10;
const MAX_INVALID_ATTEMPTS: u8 = 3;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    let mut valid_attempts: u8 = 0;
    let mut invalid_attempts: u8 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                increment_valid_attempts(&mut valid_attempts);
                num
            }
            Err(_) => {
                println!("Please enter a valid integer between 1 and 100 (inclusive).");
                increment_invalid_attempts(&mut invalid_attempts);
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! The secret number was {secret_number}.");
                break;
            }
        }
    }
}

fn increment_attempts(attempts: &mut u8, max_attemps: u8, attempt_type: &str) {
    if *attempts < max_attemps - 1 {
        *attempts += 1;
    } else {
        println!(
            "You've reached the maximum number of {} attempts ({}). Game over!\nThanks for playing!",
            attempt_type, max_attemps
        );
        std::process::exit(0);
    }
}

fn increment_valid_attempts(attempts: &mut u8) {
    increment_attempts(attempts, MAX_VALID_ATTEMPTS, "valid");
}

fn increment_invalid_attempts(attempts: &mut u8) {
    increment_attempts(attempts, MAX_INVALID_ATTEMPTS, "invalid");
}
