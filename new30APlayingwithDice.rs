use std::io;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut temp = input.split_whitespace();
    let a : i64 = temp.next().unwrap().parse().unwrap();
    let b : i64 = temp.next().unwrap().parse().unwrap();

    let mut a_wins : i64 = 0;
    let mut a_draws : i64 = 0;
    let mut b_wins : i64 = 0;

    for i in 1..7{
        let diff_a = (a-i).abs();
        let diff_b = (b-i).abs();

        if diff_a < diff_b{
            a_wins+=1;
        } 
        else if diff_a == diff_b{
            a_draws+=1;
        }
        else {
            b_wins+=1;
        }
    }

    println!("{} {} {}" ,a_wins, a_draws, b_wins);



}