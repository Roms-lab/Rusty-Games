use std::io;
use std::process::Command; 
use std::io::Write; 

fn print_credits() { 
    println!(" === Lunar 0.0.1 ===");
    println!("=== Copyright 2025 ===");
}

fn clear() { 
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .expect("Failed to clear screen on Windows");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear screen on Unix/Linux/macOS");
    }
}
 
fn main() {
    println!("=== Lunar Terminal ===");
    println!(" === Rust Editon ===");
    loop {
        let mut command = String::new();
        print!("> "); 
        
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut command)
            .expect("Error. Could not read line.");

        match command.trim() {
            "Lunar -v" => print_credits(),
            "lunar -v" => print_credits(),
            "clear" => clear(), 
            "exit" => break, 

            _ => println!("Unknown command: {}", command.trim()),
        }
    }
}