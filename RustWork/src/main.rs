use std::io;

fn array_adder(array : &[i32]) -> i32 {
   array.iter().sum()// return after all the value has been added. 
}

fn main() {
    let mut n = String::new();
    println!("Enter the number of values you want in the array: ");
    io::stdin().read_line(&mut n).unwrap();
    let number: i32 = n.trim().parse().unwrap();
 
    let mut array: Vec<i32> = Vec::new();
 
    for i in 0..number {
        // We use an infinite loop that only breaks when input is valid
        let taker: i32 = loop {
            println!("Enter value {}: ", i + 1);
            
            let mut input = String::new(); 
            io::stdin().read_line(&mut input).unwrap();
            
            // 1. Trim the input to remove newlines/spaces
            let trimmed = input.trim();

            // 2. Check if the user left it completely empty
            if trimmed.is_empty() {
                println!("Input cannot be empty! Please enter a number.");
                continue; // Skip the rest of this inner loop and ask again
            }

            // 3. Try to parse. Instead of unwrap(), we match the Result
            match trimmed.parse::<i32>() {
                Ok(num) => break num, // Success! Break the inner loop and return the number
                Err(_) => {
                    println!("That's not a valid number. Try again!");
                    // Loop continues automatically
                }
            }
        };

        // Once the inner loop breaks and gives us 'taker', we push it
        array.push(taker);
    }
    println!("Values : {:?}", array);
    println!("The total value of the array is : {}" , array_adder(&array))
}