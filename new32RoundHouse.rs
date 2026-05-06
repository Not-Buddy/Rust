use std::io;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let vec : Vec<i64> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

    let (n, a, b) = (vec[0], vec[1], vec[2]);

//    println!("{}",((a-1+b)%n+n)%n+1);
    let result = (a - 1 + b).rem_euclid(n) + 1;
    println!("{}" ,result)

}