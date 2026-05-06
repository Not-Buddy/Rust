use std::io;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let l1 : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().expect("Failed to parse"))
    .collect();

    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let l2 : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().expect("Failed to parse"))
    .collect();

    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n : i32 = input.split_whitespace().next().expect("Failed to read").parse().expect("Failed to parse");

    let mut cupsum : i32 = 0;

    let mut medsum : i32 = 0;

    for i in 0..l1.len(){
        cupsum += l1[i];
    }

    for i in 0..l2.len(){
        medsum += l2[i];
    }

  
    let shelves_for_cups = (cupsum + 5 - 1) / 5;
    let shelves_for_medals = (medsum + 10 - 1) / 10;

    if shelves_for_cups + shelves_for_medals <= n{
        println!("YES");
    }
    else {
        println!("NO");
    }

}