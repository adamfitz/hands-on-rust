use std::io::stdin;

fn main() {
    let mut visitor_list = vec![
            Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
            Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
            Visitor::new("fred", "Wow, who invited Fred?"),
        ];
    
    loop {
        println!("Hello, whats your name?");
        let name = what_is_your_name();

        // .iter () creates an iterator
        // .find returns the result of a closure, if the result is true then the matching value is returned
        // find returns an option type
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        // operate a match on teh variable returned from the iterator and find
        match known_visitor {
            // check if the option has data and make available in this clause (visitor)
            // the => is the code to execute for this match
            Some(visitor) => visitor.greet_visitor(),
            // if nothing returned, execute the println! macro
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, "New Friend"));
                }
            }
        }
    }

    println!("Thee final list of visitors is:\n{:#?}", visitor_list)
}
#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    // function chaining, result passed from one to another
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    // trim removes
    your_name.trim().to_lowercase()
}
