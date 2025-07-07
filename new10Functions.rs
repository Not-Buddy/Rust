use std::io;

fn read_int() -> u16
{
    let mut strung : String = String::new();
    io::stdin().read_line(&mut strung).expect("Failed to read line ");
    let x : u16 = strung.trim().parse().expect("Failed to convert to u16");
    return x;
}

fn read_vector() -> Vec<u16>
{
    let mut strung : String = String::new();
    io::stdin().read_line(&mut strung).expect("Failed to read line while filling vector");
    let vec : Vec<u16> = strung.split_whitespace().map(|i| i.parse().expect("Please enter valid integers")).collect();
    return vec;
}
fn main()
{
    //Rust way using map
    let x = read_int();
    let vec : Vec<u16> = read_vector();
    for ele in vec 
    {
        println!("Elements are {}",ele);
    }
    //Cpp way gotta know length for this
    let size = 3;
    let mut vec : Vec<u16> = Vec::new();
    for i in 0..size
    {
        vec.push(read_int());
    }
    println!("Vector below the cpp way i do it {:?}",vec)
    
}