use rand;
use std::cmp::Ordering;
use std::io;

fn main() {
    let number: u8 = rand::random_range(1..=100);
    println!("Welcome to Number Guessing Game");
    // println!("Guess")
    loop {
        let mut input = String::new();
        println!("Please guess a number between 0 to 100:");
        match io::stdin().read_line(&mut input) {
            Ok(_) => println!("You guessed, {input}"),
            Err(_) => {
                println!("Invalid input! Try again.");
                continue;
            }
        }
        // Convert input into integer
        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! Try again.");
                continue;
            }
        };
        match input.cmp(&number) {
            Ordering::Less => println!("\rInput too small! Try higher."),
            Ordering::Greater => println!("\rInput too large! Try lower."),
            Ordering::Equal => {
                println!("You guessed correct, {number}");
                break;
            }
        }
    }
}
