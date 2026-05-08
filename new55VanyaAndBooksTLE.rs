use std::io;
 
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
 
    let mut temp = input.split_whitespace();
    let n : i64 = temp.next().unwrap().parse().unwrap();    
    
    let mut total_digits : i64 = 0;
    for i in 1..=n{
        let mut temp = i ;
        while temp > 0{
            temp /= 10;
            total_digits += 1;
        }
    }
    println!("{}",total_digits);
    
}