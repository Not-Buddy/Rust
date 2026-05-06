use std::io;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let _n : i64 = input.split_whitespace().next().unwrap().parse().unwrap();
    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let vec : Vec<i64> = input.split_whitespace()
    .map(|s| s.parse::<i64>().unwrap())
    .collect();
    input.clear();

    //chest ,bicep ,back

    let (mut chest,mut bicep,mut back) = (0, 0, 0);

    for i in 0..vec.len(){
        let index = i%3;
        match index{
         0 => chest+=vec[i],
         1 => bicep+=vec[i],
         2 => back+=vec[i],
         _ => (),
        }
    }

    if chest > bicep && chest > back {
        println!("chest");
    } else if bicep > chest && bicep > back {
        println!("biceps");
    } else {
        println!("back");
    }

}