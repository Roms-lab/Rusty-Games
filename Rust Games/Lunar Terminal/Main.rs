use std::io;
use std::process::Command; 
use std::io::Write; 

fn print_credits() {
    println!("");
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

fn list_cmd() {
    println!("Lunar -v / lunar -v -> Prints Credits");
    println!("clear -> Clears console");
    println!("exit -> Exits the Lunar Terminal");
    println!("listcmd -> Lists all commands");
    println!("os -> Prints current os");
}

fn os() {
    if cfg!(target_os = "windows") {
        println!("Operating System: Windows");
    } else if cfg!(target_os = "macos") {
        println!("Operating System: macOS");
    } else if cfg!(target_os = "ios") {
        println!("Operating System: iOS");
    } else if cfg!(target_os = "linux") {
        println!("Operating System: Linux");
    } else if cfg!(target_os = "android") {
        println!("Operating System: Android");
    } else if cfg!(target_os = "freebsd") {
        println!("Operating System: FreeBSD");
    } else if cfg!(target_os = "dragonfly") {
        println!("Operating System: Dragonfly BSD");
    } else if cfg!(target_os = "openbsd") {
        println!("Operating System: OpenBSD");
    } else if cfg!(target_os = "netbsd") {
        println!("Operating System: NetBSD");
    } else if cfg!(target_os = "solaris") {
        println!("Operating System: Solaris");
    } else if cfg!(target_os = "illumos") {
        println!("Operating System: Illumos");
    } else if cfg!(target_os = "fuchsia") {
        println!("Operating System: Fuchsia");
    } else if cfg!(target_os = "haiku") {
        println!("Operating System: Haiku");
    } else if cfg!(target_os = "aix") {
        println!("Operating System: AIX");
    } else if cfg!(target_os = "tvos") {
        println!("Operating System: tvOS");
    } else if cfg!(target_os = "watchos") {
        println!("Operating System: watchOS");
    } else if cfg!(target_os = "emscripten") {
        println!("Operating System: Emscripten");
    } else if cfg!(target_os = "wasi") {
        println!("Operating System: WASI");
    } else if cfg!(target_os = "unknown") {
        println!("Operating System: Unknown/Embedded (None)");
    } else {
        // This covers less common or custom targets like "hermit", "redox", "vxworks", etc.
        println!("Operating System: Other or custom OS");
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
            "listcmd" => list_cmd(),
            "os" => os(),

            _ => println!("Unknown command: {}", command.trim()),
        }
    }
}
