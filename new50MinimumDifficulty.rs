use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n : usize = input.trim().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).unwrap();
    let mut vec : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse::<i32>().unwrap())
    .collect();

    //Choose which to remove first we have to remove the smallest difference once 
    let mut minimum = i32::MAX;
    let mut ordinate : usize = 1;
    for i in 1..n-1{
        let diff = vec[i+1] - vec[i-1];
        if diff < minimum {
                minimum = diff;    
                ordinate = i;
        }
    }

    vec.remove(ordinate);

    let mut maximum = i32::MIN;
    for i in 0..vec.len()-1{
        let diff = vec[i+1] - vec[i];
        if diff > maximum {
            maximum = diff;
        }
    }
    println!("{}",maximum);

}