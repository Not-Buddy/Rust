use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut temp = input.split_whitespace();
    let _n : i32 = temp.next().unwrap().parse().unwrap();
    let mut k : i32 = temp.next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let vec : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();

    k = vec[(k-1)  as usize];

    let mut advance : i32 = 0;
    for i in 0..vec.len(){
        if vec[i] >= k && k > 0{
            advance += 1;
        }
    }
    println!("{}", advance);
}