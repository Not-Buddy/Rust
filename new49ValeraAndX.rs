use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n : usize = input.trim().parse().unwrap();
    input.clear();

    let mut vec : Vec<Vec<char>> = Vec::new();
    for _ in 0..n{
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let temp : Vec<char> = input.trim().chars().collect();
        vec.push(temp);
    }

    

    let symbol : char = vec[0][0];
    let other_symbol : char = vec[0][1];

    if symbol == other_symbol {
        println!("NO");
        return;
    }

    for i in 0..n {
        if vec[i][i] != symbol || vec[i][n-1-i] != symbol {
            println!("NO");
            return;
        }
    }

    let mut count : usize = 0;

    for i in vec{
        for j in i{
            if j == other_symbol{
                count+=1;
            }
        }
    }

    let temp = n*n - (2*n-1);

    if temp == count{
        println!("YES");
    }
    else{
        println!("NO");
    }

    

}