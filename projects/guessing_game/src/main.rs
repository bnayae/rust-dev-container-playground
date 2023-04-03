use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    let chosen_number = rand::thread_rng().gen_range(1..=10);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess_number: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };

    match guess_number.cmp(&chosen_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

