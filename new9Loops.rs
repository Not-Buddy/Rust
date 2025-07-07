use std::io;
fn read_int() -> u32
{
    let mut strung: String = String::new();
    io::stdin().read_line(&mut strung).expect("Failed to read string");
    let x: u32 = strung.trim().parse().expect("Faield to convert to int");
    return x;
}
fn main()
{
    let sample = read_int();
    let nums : Vec<u32> = vec![1,2,3,4,5,6,7,8,22];
    for i in 0..sample
    {
        println!("Iteration number {}",i);
    }
    
    for ele in &nums
    {
        println!("The elements are {}",ele);
    }

    let mut i=1;
    while i!=nums.len()
    {
        println!("{}",nums[i]);
        i+=1;
    }
}