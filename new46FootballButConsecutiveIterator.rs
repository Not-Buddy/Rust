use std::io;
use std::cmp;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim();

    if s.is_empty() {
        println!("0");
        return;
    }

    let mut max_streak = 0;
    let mut current_streak = 0;
    let mut last_char = ' '; 
    for c in s.chars() {
        if c == last_char {
            current_streak += 1;
        } else {
            current_streak = 1;
            last_char = c;
        }
        
        max_streak = cmp::max(max_streak, current_streak);
    }

    println!("Longest streak: {}", max_streak);
    
    if max_streak >= 7 {
        println!("YES");
    } else {
        println!("NO");
    }
}