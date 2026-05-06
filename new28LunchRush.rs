use std::io;
use std::cmp::max;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut temp = input.split_whitespace();
    let n : i64 = temp.next().unwrap().parse().unwrap();
    let k : i64 = temp.next().unwrap().parse().unwrap();
    input.clear();

    //let mut rabbits : Vec<(i32, i32)> = Vec::new(); 
    let mut max_val : i64 = i64::MIN;
    for _ in 0..n{
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut rabbit = input.split_whitespace().flat_map(|s| s.parse::<i64>());


        let (fi, ti) = (rabbit.next().unwrap(),rabbit.next().unwrap());
        
        if ti > k{
            max_val = max(max_val, fi - (ti-k));
        }
        else {
            max_val = max(max_val, fi);
        }
    }

    println!("{}", max_val);



}