use std::io;
use std::cmp::*;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let temp : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().expect("Failed to parse"))
    .collect();
    
    let (a, b, c) = (temp[0], temp[1], temp[2]);

    let min : i32 = min(2*a+2*b, a+b+c);

    println!("{}",min);
    
}