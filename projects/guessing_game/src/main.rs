use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let v = vec![1, 2, 3];

    let mut quit: bool = false;
    while !quit {
        let chosen_number = rand::thread_rng().gen_range(1..=10);

        for _ in &v {
            let mut guess = String::new(); // MUST BE INSIDE THE LOOP

            println!("Please input your guess.");
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
            // let guess_number: i32 = guess.trim().parse().expect("Invalid input");
            let guess_number = guess.trim().parse();
            // switch into shadow variable
            let guess_number: i32 = match guess_number {
                Ok(num) => num,
                Err(_) => continue,
            };

            match guess_number.cmp(&chosen_number) {
                Ordering::Less => println!("Too small!, for '{guess_number}'"),
                Ordering::Greater => println!("Too big!,  for '{guess_number}'"),
                Ordering::Equal => {
                    println!("You win!");
                    quit = true;
                    break;
                }
            }
        }
        if !quit {
            println!("The number was {chosen_number}");
            println!("Reguess a new number!");
        }
    }
}
