use std::io;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n : i32 = input.split_whitespace().next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let s : String = input.trim().to_string();
    
    input.clear();

    let mut max = -1;
    let mut result = String::new();

    for i in 0..s.len()-1{
        let current = &s[i..i+2];
        let mut current_count = 0;

        for j in 0..s.len()-1{
            if current == &s[j..j+2]{
                current_count += 1;
            }
        }

        if current_count > max {
            max = current_count;
            result = current.to_string();
        }
    }
    println!("{}", result);
}