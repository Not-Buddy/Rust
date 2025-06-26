fn main() {
    // 1. 'loop' - Infinite loop until explicitly broken
    println!("Example of 'loop': Counting to 3");
    let mut count = 0;
    loop {
        count += 1;
        println!("Count: {}", count);
        if count == 3 {
            println!("Reached 3, breaking loop");
            break; // Exit the loop
        }
    }
    println!(); // Empty line for readability

    // 2. 'while' - Loop while a condition is true
    println!("Example of 'while': Counting down from 5");
    let mut counter = 5;
    while counter > 0 {
        println!("Counter: {}", counter);
        counter -= 1;
    }
    println!("Countdown finished!");
    println!(); // Empty line for readability

    // 3. 'for' - Iterate over a range or collection
    println!("Example of 'for': Iterating over a range (1 to 5)");
    for i in 1..6 { // Range from 1 to 5 (exclusive of 6)
        println!("Number: {}", i);
    }
    println!(); // Empty line for readability

    // 'for' with an array
    println!("Example of 'for': Iterating over an array");
    let fruits = ["Apple", "Banana", "Orange"];
    for fruit in fruits.iter() { // Iterate over references to array elements
        println!("Fruit: {}", fruit);
    }
    println!(); // Empty line for readability

    // Demonstrating 'continue' and 'break' in a loop
    println!("Example of 'continue' and 'break': Printing odd numbers up to 7");
    for num in 1..10 {
        if num % 2 == 0 {
            continue; // Skip even numbers
        }
        println!("Odd number: {}", num);
        if num == 7 {
            break; // Stop loop at 7
        }
    }
}


fn main()
{

    let mut count=0;
    loop
    {
        cout+=1;
        println!("Count right now: {}",count);
        if count == 2
        {
            break;        
        }  
        
        
    }
}
