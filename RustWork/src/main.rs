use std::{
    fs::File,
    io::{self, Write},
};

fn main() {
    println!("Enter file name:");

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).unwrap();

    let file_name = file_name.trim();
    let full_path = format!("{}.txt", file_name);

    let mut file = File::create(full_path).unwrap();

    println!("Type something (type 'exit' to stop)");

    loop {
        let mut values = String::new();

        io::stdin().read_line(&mut values).unwrap();

        if values.trim() == "exit" {
            break;
        }

        file.write_all(values.as_bytes()).unwrap();
    }

    println!("Done!");
}