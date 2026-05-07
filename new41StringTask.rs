use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let temp : String = input.trim().to_lowercase().parse().unwrap();

    let vec : Vec<char> = temp.chars().collect();

    let mut strung : String = String::new();

    for i in 0..vec.len(){
        if vec[i] == 'a' || vec[i] == 'e' ||
        vec[i] == 'i' || vec[i] == 'o' ||
        vec[i] == 'u'{
            continue;
        }
        else {
            strung.push('.');
            strung.push(vec[i]);
        }
    }

    println!("{}",strung);
}