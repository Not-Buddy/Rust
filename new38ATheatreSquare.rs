use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let vec : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();

    let (n, m, a) = (vec[0], vec[1], vec[2]);

    let width = (n as f64 / a as f64).ceil() as i64;
    let height = (m as f64 / a as f64).ceil() as i64;

    let output = width * height;

    println!("{}" ,output);


}