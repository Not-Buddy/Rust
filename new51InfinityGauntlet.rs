use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n : usize = input.trim().parse().unwrap();
    input.clear();

    let mut strungs : Vec<String> = Vec::new(); 
    let mut not_there : Vec<String> = Vec::new();

    for _ in 0..n{

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let strung : String = input.trim().parse().unwrap();
        
        strungs.push(strung);
    }   

    let all_stones = [("purple","Power"),("green","Time"),("blue","Space"),
    ("orange","Soul"), ("red", "Reality"), ("yellow", "Mind")];

    for (color, stone) in all_stones{
        if !strungs.contains(&color.to_string()){
            not_there.push(stone.to_string());
        }
    }

    println!("{}",not_there.len());
    for i in not_there{
        println!("{}",i);
    }


}