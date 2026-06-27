use std::io;


pub fn main() {
    let value = "I love learning rust";
    let mut string_value = String::new();
    println!("Enter a word you wanna find: ");
    io::stdin().read_line(&mut string_value).unwrap();
    
        
    match value.find(string_value.trim()) {
        Some(index) => {
            println!("The value was found at : {}" , index);
        }
        None => {
            println!("No such value exist. ")
        }
    }
}