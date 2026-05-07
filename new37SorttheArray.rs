use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n : usize = input.split_whitespace().next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut vec : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();
    input.clear();

    let mut vec2 : Vec<i32> = vec.clone();
    vec2.sort();

    let mut l = 0;
    let mut r = vec.len()-1;
    for i in 0..vec.len()-1{
        if vec[i] > vec[i+1]{
            l = i;
            break;
        }
    }

    for i in (0..n).rev(){
        if i>0 && vec[i] < vec[i-1]{
            r = i;
            break;
        }
    }

    vec[l..r + 1].reverse();

    if vec == vec2{
        println!("yes");
        println!("{} {}", l+1, r+1);
    }
    else{
        println!("no");
    }
}