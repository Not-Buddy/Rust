use std::io;
use std::cmp::*;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut temp = input.split_whitespace();
    let _n : i32 = temp.next().unwrap().parse().unwrap();    
    let mut k : i32 = temp.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let vec : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();

    let mut steps : i32 = 0;

    for i in &vec{
        let tp = min(&8,&i);
        k-=tp;
        steps += 1;
        if k<=0{
            break;
        }
    }

    if k <= 0{
        println!("{}",steps);
    }
    else {
        println!("-1");
    }
    
}