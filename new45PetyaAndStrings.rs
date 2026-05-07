use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut vec1 : Vec<char> = input.trim().to_lowercase().chars().collect();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let mut vec2 : Vec<char> = input.trim().to_lowercase().chars().collect();
    input.clear();

    vec1.sort();
    vec2.sort();

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