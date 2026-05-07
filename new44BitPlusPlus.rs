use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = iter.next().unwrap();

    let mut x : i32 = 0;
    for _ in 0..n{
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let s : Vec<char> = input.trim().chars().collect();

        if s.contains(&'+'){
            x += 1;
        }
        else {
            x -= 1;
        }
    }

    println!("{}",x);

}