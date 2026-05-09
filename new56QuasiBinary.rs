use std::io;
 
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
 
    let n : String = input.trim().to_string();
    //32
    
    let k = n.chars()
    .map(|s| s.to_digit(10).unwrap())
    .max().unwrap();
    // ['3','2'] -> after map [3,2] after max 3
    println!("{}",k);

    let mut results = vec![0; k as usize];
    //[0,0,0]

    let mut power = 1;
    for c in n.chars().rev(){ // ['2', '3'] as its reversed
        let digit = c.to_digit(10).unwrap() as usize; // digit = 2 on first iteration

        for i in 0..digit{ // 0..2
            results[i] += power; // results [1,1,0] after first iteration 
            // results [11, 11, 10] after second iteration
        }
        power *= 10;
    }

    for (i, val) in results.iter().enumerate(){
        print!("{}{}" ,if i > 0 {" "} else {""},val)
    }
    println!();
    
}