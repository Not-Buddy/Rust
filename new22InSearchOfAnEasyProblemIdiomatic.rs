use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();

    // Use .any() to check for the condition directly
    let is_hard = input
        .split_whitespace()
        .any(|s| s == "1"); // Check string equality before even parsing!

    if is_hard {
        println!("HARD");
    } else {
        println!("EASY");
    }
}