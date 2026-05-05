use std::io;

fn main(){
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2)
    .expect("Failed to read line");

    let n : i32 = input2.split_whitespace().next().unwrap().parse().unwrap();

    input2.clear();

    io::stdin().read_line(&mut input2)
    .expect("Failed to read line");
    
    let temp : Vec<i32> = input2.split_whitespace()
    .map(|s| s.parse().expect("Failed to parse"))
    .collect();


    let mut flag : bool = false;
    for i in 0..temp.len(){
        if temp[i] == 1{
            flag = true;
        }
    }

    if flag{
        println!("HARD");
    }
    else {
        println!("EASY");
    }


}