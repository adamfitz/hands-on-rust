use std::io::stdin;

fn main() {
    let mut visitor_list = vec![
            Visitor::new("bert", VisitorAction::Accept, 45),
            Visitor::new("steve", VisitorAction::AcceptWithNote{ note: String::from("Lactose-free milk is in the fridge")}, 15),
            Visitor::new("fred", VisitorAction::Refuse, 30),
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
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }
    }

    println!("Thee final list of visitors is:\n{:#?}", visitor_list)
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote {note: String},
    Refuse,
    Probation,
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8
}

impl Visitor {
    // function follows the constructor (struct Visitor) pattern, accepts params for the contents of the struct and returns self
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to teh treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do NOT allow {} in!", self.name)
        }
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
