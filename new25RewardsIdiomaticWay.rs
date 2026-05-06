use std::io;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let cups_sum : i32 = input.split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .sum();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let meds_sum : i32 = input.split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .sum();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let n : i32 = input.trim().parse().unwrap();

    let shelves_for_cups = (cups_sum + 5 - 1) / 5;
    let shelves_for_medals = (meds_sum + 10 - 1) / 10;
    
    if shelves_for_cups + shelves_for_medals <= n{
        println!("YES");
    }
    else {
        println!("NO");
    }
}