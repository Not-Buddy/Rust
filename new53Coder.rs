use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut temp = input.split_whitespace();
    let n = temp.next().unwrap().parse().unwrap();    
    
    let mut flag : bool = true;
    let mut count : i32 = 0;
    let mut pattern : String = String::new();

    for _ in 0..n{
        for _ in 0..n{
            if flag == true{
                pattern.push('C');
                count += 1;
            }
            else {
                pattern.push('.');
            }
            flag = !flag;
        }
        pattern.push('\n');
        if n%2==0{
            flag = !flag;
        } 
    }

    println!("{}",count);
    println!("{}",pattern);
}