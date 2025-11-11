use std::io::{self, Write};
use rand::Rng; // Import Rng trait

fn main() {
    // Generate the secret number first
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..=100);

    println!("|===============|");
    println!("| Guessing Game |");
    println!("|===============|");
    println!("(Secret number is: {})", secret_number); // For testing

    print!("Number -> ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // You can now compare 'guess' (after parsing it to a number)
    // with 'secret_number' to implement your game logic.
    println!("You guessed: {}", guess);
}