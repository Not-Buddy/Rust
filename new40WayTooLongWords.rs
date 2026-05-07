use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut temp = input.split_whitespace();
    let n : i32 = temp.next().unwrap().parse().unwrap();
    input.clear();
    
    let mut strungs : Vec<String> = Vec::new();
    for _ in 0..n{
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let strung : String = input.parse().unwrap();
        strungs.push(strung);
    }

    for i in strungs{
        if i.len()<10{
            println!("{}",i);
        }
        else {
            let mut temp = i.trim().chars();
            let first = temp.next().unwrap();
            let last = temp.last().unwrap();
            println!("{}{}{}",first ,i.len()-3 ,last);
        }
    }

}