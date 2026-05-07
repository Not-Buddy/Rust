// Co ordinate compression technique using rust HashMap
use std::collections::HashMap;
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

    let mut sorted_vec = vec.clone();
    sorted_vec.sort_unstable();

    let mut rank_map = HashMap::new();
    for (index, &value) in sorted_vec.iter().enumerate(){
        rank_map.entry(value).or_insert(index);
    }

    for x in &mut vec{
        *x = *rank_map.get(x).unwrap() as i32;
    }

    let mut l : i32 = -1;
    let mut r : i32 = -1;

    for i in 0..n{
        if vec[i] != i as i32 {
            if l == -1 {
                l = i as i32;
            }
            r = i as i32;
        }
    }

    if l == -1 {
        println!("yes");
        println!("1 1");
        return;
    }

    let left = l as usize;
    let right = r as usize;

    vec[left..right + 1].reverse();

    let mut is_sorted = true;
    for i in 0..n {
        if vec[i] != i as i32 {
            is_sorted = false;
            break;
        }
    }

    if is_sorted {
        println!("yes");
        println!("{} {}",left+1 ,right+1);
    }
    else {
        println!("no");
    }




}