use std::io;

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let l1 : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().expect("Failed to parse"))
    .collect();

    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let l2 : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().expect("Failed to parse"))
    .collect();

    input.clear();

    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n : i32 = input.split_whitespace().next().expect("Failed to read").parse().expect("Failed to parse");

    let mut cupsum : i32 = 0;

    let mut medsum : i32 = 0;

    for i in 0..l1.len(){
        cupsum += l1[i];
    }

    for i in 0..l2.len(){
        medsum += l2[i];
    }

    if n<=0 && cupsum<=0 && medsum<=0 {
        println!("NO");
    }
    else if n==1 && cupsum == 0 || medsum == 0{
        println!("YES");
    } 
    else if n>=2{

        let maxcups = n*5;
        let maxmeds = n*10;

        if maxcups + maxmeds <= cupsum + medsum {
                    println!("YES");
        }
        else {
            println!("NO");
        }

    }
    else{
        println!("NO");
    }


}