fn main() {
    let num = 10;
    let parse = num.to_string();
    let added = parse + " " + "Sarthak";
    println!("The value is a string: {}", added);

    let tupple = ("Apple" , 12 , true , 13.75); // being able to store different data type
    println!("The tupple will run :{:?} ", tupple);
    let statement = [1,2,3,4,5,6,7];
    for stmt in statement {
        print!("{stmt}\n");
    }
}
