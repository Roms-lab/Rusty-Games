use std::io;
use std::cmp::Ordering;
use rand::Rng; // Trait that defines the random generation method

fn main() {
    println!("Guess the number!");

    // Generate a secret number between 1 and 100 (inclusive on both ends)
    let secret_number = rand::thread_rng().gen_range(1..=100); 

    // Uncomment the next line for debugging/testing
    // println!("The secret number is: {secret_number}");

    loop { // Start an infinite loop for the game
        println!("Please input your guess:");

        // Create a mutable string to store user input
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // Handle potential failure

        // Convert the guess from a String to a u32 (unsigned 32-bit integer)
        // We use a `match` expression to handle potential errors (like non-numeric input)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // If parsing is successful, use the number
            Err(_) => { // If parsing fails (e.g., user enters "two"), skip this iteration and ask again
                println!("Please enter a valid number!");
                continue;
            }
        };

        // Compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // Exit the loop (and the game) on a correct guess
            }
        }
    }
}
