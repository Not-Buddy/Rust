use std::io;
use std::cmp::*;

fn main(){

    // 1:2:4
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error reading line");
    let a : i32 = input.trim().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Error reading line");
    let b : i32 = input.trim().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Error reading line");
    let c : i32 = input.trim().parse().unwrap();
    input.clear();
    
    let min = min(min(b/2,c/4),a);
    println!("{}",min*7);
    
}