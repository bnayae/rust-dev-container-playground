use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let chosen_number = rand::thread_rng().gen_range(1..=10);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Number is {chosen_number}, You guessed: {guess}");
}