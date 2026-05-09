use std::io;
use std::cmp::*;
 
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let mut temp = input.split_whitespace();
    let n : usize = temp.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let mut vec : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();

    if n<2{
        println!("0");
        return;
    }

    vec.sort();

    let instab1 = vec[n-1] - vec[1];
    let instab2 = vec[n-2] - vec[0];

    println!("{}",min(instab1, instab2));
    
}