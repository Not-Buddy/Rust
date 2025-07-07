use std::io;


fn print_type_of<T>(_: &T) 
{
    println!("{}", std::any::type_name::<T>());
}

fn main()
{
    let animals = ["bird", "frog", "cat"];
    let uppercased: Vec<_> = animals.iter().map(|s| s.to_uppercase()).collect();
    println!("{:?}", uppercased); // Output: ["BIRD", "FROG", "CAT"]

    let LomaDeath : Vec<u32> = vec![1,2,2,2,2,2,2,3,3,4,6,6,6,6,64,4];
    let transformation : Vec<u32> = LomaDeath.iter().map(|i| i*i).collect();
    println!("{:?}",transformation);

    let letters = ['a', 'b', 'c'];
    let mut count = 0;
    let mut offset = 0;
    let pairs: Vec<_> = letters
    .iter()
    .map(|&letter| {
        count += 1;
        offset +=2;
        (letter, count, offset)
    })
    .collect();

    print_type_of(&pairs[0]);
    println!("{:?}", pairs); // Output: [('a', 1), ('b', 2), ('c', 3)]


}
