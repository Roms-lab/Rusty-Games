use std::{thread, time::Duration};

fn main() {
    println!("--- Gravity Simulation ---");
    let gravity = 9.81;
    let mut speed = 9.81;
    while true {
        thread::sleep(Duration::from_secs(1));
        println!("Current Gravity: {} m/s^2", speed );
        speed += gravity;
    }
}