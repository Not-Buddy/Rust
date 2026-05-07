use std::collections::HashMap;

fn main(){
    
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score{
        Some(s) => println!("The score is {}" ,s),
        None => println!("Team not found"),
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    //insering words in hashmap
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let strung : String = "Hello there my name i Aary".to_string();
    let mut hash : HashMap<char, i32> = HashMap::new();

    for c in strung.to_lowercase().chars(){
        let count_ptr = hash.entry(c).or_insert(0);
        *count_ptr += 1;
    }

    println!("Alphabet counts:");
    for (letter, count) in &hash{
        println!("{}: {}",letter,count);
    }

}