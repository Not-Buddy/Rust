use std::io;
use std::collections::HashMap;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut input2 = input.split_whitespace();
    let _n : i32 = input2.next().unwrap().parse().unwrap();
    let m : i32 = input2.next().unwrap().parse().unwrap();
    input.clear();

    let mut dict = HashMap::new();

    for _ in 0..m{
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let words: Vec<String> = input.split_whitespace()
        .map(|s| s.to_string())
        .collect();

        let first = &words[0];
        let second = &words[1];

        if second.len() < first.len(){
            dict.insert(first.clone(), second.clone());
        }
        else {
            dict.insert(first.clone(), first.clone());
        }
    }
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let lecture: Vec<String>  = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

        let mut result = Vec::new();
        for word in lecture{
            let shorter_word = dict.get(&word).unwrap();
            result.push(shorter_word.clone());
        }


        println!("{}", result.join(" "));

        


    // let mainvec : Vec<Vec<String>> = Vec::new();
    // for i in 0..m{

    //     input.clear();
    //     io::stdin().read_line(&mut input).expect("Failed to read line");
    //     let vec : Vec<String> = input.split_whitespace()
    //     .map(|s| s.parse().unwrap())
    //     .collect();

    //     mainvec.push(vec);
    // }

    // input.clear();
    // io::stdin().read_line(&mut input).expect("Failed to read line");
    // let vec : Vec<String> = input.split_whitespace()
    // .map(|s| s.parse().unwrap())
    // .collect();

    // for i in 0..n{

    //     find(vec[i] in mainvec){
    //         if found compare mainvec[0] with mainvec[1]
    //         and push back the smaller one if same length then push back 0 one
    //     }

    // }




}