use std::io;
fn main()
{
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    println!("Output without unwrapping: {}",input);
}
