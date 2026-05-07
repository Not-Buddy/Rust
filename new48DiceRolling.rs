use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n : i32 = input.trim().parse().unwrap();
    input.clear();

    let mut vec : Vec<i32> = Vec::new();
    for _ in 0..n{
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let temp : i32 = input.trim().parse().unwrap();
        vec.push(temp);
    }

    for i in vec{
        if i>=2 && i<=7{
            println!("1");
        }
        else {
            if i%7 != 0{
                let temp = i/7;
                println!("{}",temp+1);
            } else {
                let temp = i/7;
                println!("{}",temp);
            }
        }
    }
}