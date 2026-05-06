use std::io;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n : i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    if n%2==0 && (n!=2){
        println!("YES");
    }
    else {
        println!("NO");
    }

}