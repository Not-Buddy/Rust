use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut temp = input.split_whitespace();
    let n = temp.next().unwrap().parse().unwrap();    
    
    let mut flag : bool = true;

    println!("{}",n);
    for _ in 0..n{
        for _ in 0..n{
            if flag == true{
                print!("C");
            }
            else {
                print!(".");
            }
            flag = !flag;
        }
        println!("");
        if n%2==0{
            flag = !flag;
        } 
    }
}