use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n = input.trim().parse().unwrap();
    input.clear();

    let mut solved : i32 = 0;
    for _ in 0..n{
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let line : Vec<i32> = input.split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

        let (p,v,t) = (line[0],line[1],line[2]);

        if p+v+t >= 2{
            solved += 1;
        }
    }

    println!("{}", solved);

}