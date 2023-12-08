use std::io::{self};
use std::cmp::Ordering;
use std::ops::Index;
use colored::Colorize;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_times = 0;

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut _guess: String = String::new();


        io::stdin()
            .read_line(&mut _guess)
            .expect("Failed to read line");

        let _guess: u32 = match _guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {_guess}");

        match _guess.cmp(&secret_number) {
            Ordering::Less => {
                guess_times += 1;
                println!("{}", "Too Small!".red())
            }
            Ordering::Greater => {
                guess_times += 1;
                println!("{}","Too Big!".red())
            }
            Ordering::Equal => {
                match guess_times {
                    1..=15 => println!("Congratulations! You Finally Got It!"),
                    g if g > 15  => println!("You got it resilience is a commendable spirit!"),
                    _ => println!("How did you get it on the first try?")
                }
                break;
            },
        }
    }
}
/// .
fn response(order: &str, guess_times: i32){
    let message_index = rand::thread_rng().gen_range(0..=2);
    match guess_times {
        1..=5 => {
            let messages = vec![
                "Not quite, but you're getting there!",
                "Close, but not close enough!",
                "You're warming up!",
            ];
            let message = messages.index(message_index);
            println!("{}, your number is {}", message, order);
        }
        6..=15 => {
            let messages = vec![
                "Hmmm, not quite the answer!",
                "You're in the right ballpark!",
                "Getting closer, keep it up!",
            ];
            let message = messages.index(message_index);
            println!("{}, your number is {}", message, order);
        }
        16..=30 => {
            let messages = vec![
                "You're persistent, I'll give you that!",
                "Not quite yet, keep going!",
                "You're on a wild guessing spree!",
            ];
            let message = messages.index(message_index);
            println!("{}, your number is {}", message, order);
        }
        g if g > 30 => {
            let messages = vec![
                "Are you sure you want to continue?",
                "You must really love guessing!",
                "I admire your determination!",
            ];
            let message = messages.index(message_index);
            println!("{}, your number is {}", message, order);
        }
        _ => println!("Keep trying!"),
    }
}