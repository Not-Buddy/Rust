use std::io;
use std::cmp::*;
 
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
 
    let mut temp = input.split_whitespace();
    let n : i64 = temp.next().unwrap().parse().unwrap();    
    
    let mut start = 1;
    let mut total_digits = 0;
    let mut length = 1; 

    while start <= n{
        let end = min(n, start * 10 - 1);
        let count_range = end - start + 1;
        total_digits += count_range * length;

        start *= 10;
        length +=1;
    }

    println!("{}",total_digits);
    
}