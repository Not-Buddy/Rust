use std::io;
fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let temp : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().expect("Not a number"))
    .collect();

    let (k, n, w) = (temp[0], temp[1], temp[2]);

    let mut total_cost : i32 = 0;
    
    for i in 1..w+1{
        total_cost += i * k;
        // println!("cost {} :- {}",i,total_cost);
    }
    if total_cost <= n{
        println!("{}",0);
    }
    else {
        println!("{}",total_cost-n);
    }

}