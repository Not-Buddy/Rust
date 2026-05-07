use std::io;

// fn is_prime(n: u64) -> bool{
//     if n<=1{
//         return false;
//     }

//     let limit = (n as f64).sqrt() as u64;

//     for i in 2..=limit{
//         if n % i == 0{
//             return false;
//         }
//     }
//     true
// }

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n : i32 = input.trim().parse().unwrap();
    input.clear();



    if n % 2 == 0 && n>1{
        let num = n/2;
        println!("{}",num);
        for _ in 0..num{
            print!("2 ");
        }
    }
    else {
        let num = (n/2)-1;
        println!("{}",num+1);
        for _ in 0..num{
            print!("2 ");
        }
        print!("3 ");
    }

}