use std::io;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n : i32 = input.trim().parse().unwrap();
    input.clear();

    let mut temp: Vec<Vec<i32>> = Vec::new();
    for _ in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let row: Vec<i32> = input.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        temp.push(row);
    } 

    temp.sort_unstable();

    let mut happy = false;

    for i in 0..(n as usize) -1{
        let current_quality = temp[i][1];
        let next_quality = temp[i+1][1];

        if current_quality > next_quality{
            happy = true;
            break;
        }
    }

    if happy {
        println!("Happy Alex");
    }
    else {
        println!("Poor Alex");
    }

}