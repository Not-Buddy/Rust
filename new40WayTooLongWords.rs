use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        let s = input.trim();
        let true_len = s.len();

  
        if true_len > 10 {
            let mut chars = s.chars();
            let first = chars.next().unwrap();
            let last = chars.last().unwrap();
            
            println!("{}{}{}", first, true_len - 2, last);
        } else {
            println!("{}", s);
        }
    }
}