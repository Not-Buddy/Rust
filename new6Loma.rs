use std::io;
fn main()
{
    println!("Enter your age");
    let mut loma = Default::default();
    io::stdin().read_line(&mut loma).expect("Error reading string");
    let loma: u32 = loma.trim().parse().expect("Error Converting");
    let decision = if loma>18{"Elligible"}else{"Not Elligible"};
    println!("{}",decision);
}