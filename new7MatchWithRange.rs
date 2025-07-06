use std::io;

fn main()
{
    println!("Enter your age: ");
    let mut strung: String = String::new();
    io::stdin().read_line(&mut strung).expect("Failed to read");
    let guess: u16 =strung.trim().parse().expect("Failed to convert");

    match guess
    {
        0..=17 => println!("Not Elligible"),
        18..=100 => println!("Elligible"),
        _ => println!("Enter valid age"),
    }
}