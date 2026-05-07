use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let strung : String = input.trim().to_string();
    input.clear();

    if strung.contains("1111111") || strung.contains("0000000"){
        println!("YES");
    }
    else {
        println!("NO");
    }

}