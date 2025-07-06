use std::io;


fn read_int(prompt: &str) -> i32 {
    use std::io;
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<i32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid integer."),
        }
    }
}

enum Message
{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

fn process_manager(msg: Message)
{
    match msg 
    {
        Message::Quit => println!("Quit message received"),
        Message::Move {x,y} => println!("Move to {} , {}",x,y),
        Message::Write (text) => println!("Wrote a message {} ",text),
        Message::ChangeColor (a,b,c) => println!("Change the color to r{} g{} b{}",a,b,c),
    }
}

fn main()
{

    println!("Select a message type by entering a number:");
    println!("1: Quit");
    println!("2: Move");
    println!("3: Write");
    println!("4: ChangeColor");

    let mut strung: String = String::new();
    io::stdin().read_line(&mut strung).expect("Failed to read line");
    let integer: u16 = strung.trim().parse().expect("Failed to convert to integer enter valid integer");

    let message = match integer{
        1 => Message::Quit,
        2 => 
        {
            let x = read_int("Enter x co-ordinate");
            let y = read_int("Enter y co-ordinate");
            Message::Move{x,y}
        },
        3 => 
        {
            println!("Enter a string");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read string");
            Message::Write(input.trim().to_string())
        },
        4 => 
        {
            println!("Enter the color r g b");
            let r = read_int("Enter r");
            let g = read_int("Enter g");
            let b = read_int("Enter b");
            Message::ChangeColor(r,g,b)
        },
        _ => 
        {
            println!("Enter a valid choice");
            Message::Quit
        },
    };
    process_manager(message);
}