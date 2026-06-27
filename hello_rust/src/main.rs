trait Greet {
    fn hello(&self) -> String;
    fn goodbye(&self) -> String {
        String::from("Bye")
    }
}
struct Person {
    name: String,
}

impl Greet for Person {
    fn hello(&self) -> String {
        format!("Hello , {}", self.name)
    }
}

pub fn main() {
    let p = Person {
        name: String::from("Sarthak"),
    };
    println!("{}", p.hello());
    println!("{}", p.goodbye());
}
