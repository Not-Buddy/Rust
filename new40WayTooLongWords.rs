use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut temp = input.split_whitespace();
    let mut n : i32 = temp.next().unwrap().parse().unwrap();
    input.clear();
    
    let mut strungs : Vec<String> = Vec::new();
    for _ in 0..n{
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let strung : String = input.clone();
        strungs.push(strung);
    }

    for i in strungs{
        if i.len()<10{
            println!("{}",i);
        }
        else {
            let mut chars = i.trim().chars();
            let first = chars.next().unwrap();
            let last = chars.last().unwrap();

            let s = i.trim();
            n = s.len() as i32;

            println!("{}{}{}", first, n - 2, last);
        }

    }

}