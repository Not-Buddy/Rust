use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut temp = input.split_whitespace();
    let a : i32 = temp.next().unwrap().parse().unwrap();    
    let b : i32 = temp.next().unwrap().parse().unwrap();    
    let c : i32 = temp.next().unwrap().parse().unwrap();    

    // a is 1st term c is difference 
    // b is the term that we want it to be 

    if c == 0 {
        if a == b{
            println!("YES");
        }
        else {
            println!("NO");
        }
    }
    else {
        let gap = b - a;

        if gap % c == 0 && gap / c >= 0 {
            println!("YES");
        }
        else {
            println!("NO");
        }
    }

    
}