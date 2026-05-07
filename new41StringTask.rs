use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let temp : String = input.trim().to_lowercase();

    let vec : Vec<char> = temp.chars().collect();

    let mut strung : String = String::new();

    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];

    for i in 0..vec.len(){
        if vowels.contains(&vec[i]){
            continue;
        }
        else {
            strung.push('.');
            strung.push(vec[i]);
        }
    }

    println!("{}",strung);
}