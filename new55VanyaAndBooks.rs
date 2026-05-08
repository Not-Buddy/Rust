use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut temp = input.split_whitespace();
    let n : i32 = temp.next().unwrap().parse().unwrap();    

    let mut vec : Vec<Vec<char>> = Vec::new();

    for i in 1..=n{
        let single : Vec<char> = i.to_string().chars().collect();
        // println!("{:?}",single);
        vec.push(single);
    }

    let mut digits : i32 = 0;

    for num in vec{
        digits += num.len() as i32;
    }

    println!("{}",digits);

    
}