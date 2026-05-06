use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let s : Vec<char> = input.trim().chars().collect();

    let n = s.len();

    let mut defects : usize = 0;

    for i in 0..n/2{
        if s[i] != s[n-1-i]{
            defects+=1;
        }
    }
    if defects == 1 || (defects == 0 && n%2 == 1) {
        println!("YES");
    }
    else {
        println!("NO");
    }
}