use std::io;
use std::cmp::*;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let temp : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().expect("Failed to parse"))
    .collect();

    let (c1, c2, c3, c4, c5) = (temp[0],temp[1],temp[2],temp[3],temp[4]);

    let sum = c1 + c2 + c3 + c4 + c5;

    if sum % 5 == 0 && sum != 0 {
        println!("{}", sum / 5)
    }
    else {
        println!("-1");
    }
}