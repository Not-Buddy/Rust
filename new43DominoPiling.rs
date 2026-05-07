use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = iter.next().unwrap();
    let m = iter.next().unwrap();

    let area = n*m;
    let output = ((area/2) as f32).floor();

    println!("{}", output);

}