use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let vec1 : Vec<char> = input.trim().to_lowercase().chars().collect();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let vec2 : Vec<char> = input.trim().to_lowercase().chars().collect();
    input.clear();

    if vec1 == vec2{
        println!("0");
    }
    else if vec1 < vec2{
        println!("-1");
    }
    else {
        println!("1");
    }

}