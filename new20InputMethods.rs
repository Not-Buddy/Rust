use std::io;

fn main(){
 
    

    let mut input = String::new();

    println!("This actually works enter some string: ");
    io::stdin()
    .read_line(&mut input)
    .expect("Could not read String line");

    println!("Here is the string that you entered :- {}",input);

    input.clear();

    println!("Enter a vector of numbers to proceed:- ");
    io::stdin().read_line(&mut input).expect("couldnt read line");

    let nums : Vec<i32> = input.split_whitespace()
    .map(|s| s.parse().expect("Failed in parsing"))
    .collect();

    println!("{:?} ", nums);

    let mut input2 = String::new();


    println!("Enter 3 numbers separated by whitespace :-");
    io::stdin()
    .read_line(&mut input2)
    .expect("Error reading input2");


    let mut iter = input2.split_whitespace();

    let a : i32 = iter.next().expect("Failed to read 1st").parse().expect("Not a number");
    let b : i32 = iter.next().expect("Failed to read 2nd").parse().expect("Not a number");
    let c : i32 = iter.next().expect("Failed to read 3rd").parse().expect("Not a number");

    println!("a: {}, b: {}, c: {}",a,b,c);

    input2.clear();


    println!("Enter 3 numbers separated by whitespace");
    io::stdin()
    .read_line(&mut input2)
    .expect("Error reading input3");

    let temp : Vec<i32> = input2.split_whitespace()
    .map(|s| s.parse().expect("Failed to parse"))
    .collect();

    let (a,b,c) = (temp[0],temp[1],temp[2]);

    println!("a: {}, b: {}, c: {}",a,b,c);



}