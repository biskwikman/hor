use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_lowercase(),
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn whats_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}
 

fn main() {
    println!("Hello, what's your name?");
    let your_name = whats_your_name();
    let visitor_list = [
        Visitor::new("dan", "Hello, Dan. Come eat a macaroni salad."),
        Visitor::new("alex", "Hello, Alex. Come eat a beef salad."),
        Visitor::new("doug", "Hello, Doug. Come eat a caprese salad."),
    ];
    let known_visitor = visitor_list
        .iter()
        .find(|visitor| visitor.name == your_name);
}
