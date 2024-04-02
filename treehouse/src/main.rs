use std::io::stdin;

fn main() {
    println!("Hello, whats your name?");
    let name = what_is_your_name();
    println!("Hello {:?}", name);

    let visitor_list = ["bert", "steve", "fred"];
    let mut allow_them_in = false;

    for visitor in visitor_list {
        if visitor == &name {
            allow_them_in = true;
        }
    }

    if allow_them_in {
        println!("Welcome to the Treehouse, {}", name)
    } else {
        println!("NOPE! You NOT on the list ;)");
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
