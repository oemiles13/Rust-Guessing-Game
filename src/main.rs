use std::io::{self};
use std::cmp::Ordering;
use colored::Colorize;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

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
            Ordering::Less => println!("{}", "Too Small!".red()),
            Ordering::Greater => println!("{}","Too Big!".red()),
            Ordering::Equal => {
                println!("{}","You Win!".green());
                break;
            },
        }
    }
}