// Followed the tutorial from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MAX_VALID_ATTEMPTS: u8 = 10;
const MAX_INVALID_ATTEMPTS: u8 = 3;
const MAX_CHAOS: f64 = 2.0;

fn main() {
    println!("Guess the number!");

    let original_secret_number: u8 = rand::thread_rng().gen_range(1..=100);
    let mut current_secret_number: u8 = original_secret_number;
    let mut valid_attempts: u8 = 0;
    let mut invalid_attempts: u8 = 0;

    loop {
        let suffix = match valid_attempts + 1 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        };
        println!("Please input your {}{} guess.", valid_attempts + 1, suffix);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => {
                increment_valid_attempts(&mut valid_attempts, current_secret_number);
                shift_secret_number(&mut current_secret_number, valid_attempts);
                num
            }
            Err(_) => {
                println!("Please enter a valid integer between 1 and 100 (inclusive).");
                increment_invalid_attempts(&mut invalid_attempts, current_secret_number);
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&current_secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!(
                    "You win! The secret number was {current_secret_number} and you guessed it in {valid_attempts} attempts.\nThanks for playing!"
                );
                break;
            }
        }
    }
}

fn increment_attempts(attempts: &mut u8, max_attempts: u8, attempt_type: &str, secret_number: u8) {
    if *attempts < max_attempts - 1 {
        *attempts += 1;
    } else {
        println!(
            "You've reached the maximum number of {} attempts ({}). Game over! The secret number was {}.\nThanks for playing!",
            attempt_type, max_attempts, secret_number
        );
        std::process::exit(0);
    }
}

fn increment_valid_attempts(attempts: &mut u8, secret_number: u8) {
    increment_attempts(attempts, MAX_VALID_ATTEMPTS, "valid", secret_number);
}

fn increment_invalid_attempts(attempts: &mut u8, secret_number: u8) {
    increment_attempts(attempts, MAX_INVALID_ATTEMPTS, "invalid", secret_number);
}

fn shift_curve(x: f64) -> f64 {
    let sign = if rand::thread_rng().gen_bool(0.5) {
        1.0
    } else {
        -1.0
    };
    sign * MAX_CHAOS * f64::log10(2.0).powi(4) * (1.0 / 16.0) * x.powi(4)
}

fn shift_secret_number(current_secret_number: &mut u8, valid_attempts: u8) {
    println!("(Debug) valid_attempts: {valid_attempts}");
    let shift = shift_curve(valid_attempts as f64).round() as i8;
    let shifted_number = *current_secret_number as i8 + shift;
    *current_secret_number = shifted_number.clamp(1, 100) as u8;
}
