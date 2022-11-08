use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    //Adding values to a HashMap
    marks.insert("Human Computer Interaction", 80);
    marks.insert("DSA",78);
    marks.insert("System Design",90);
    marks.insert("OOAD", 85);

    //Find length of HashMap
    println!("You have done {} subjects", marks.len());

    //Get a value from HashMap
    match marks.get("DSA"){
        Some(mark) => println!("You scored {}% in DSA",mark),
        None => println!("You did not study DSA!")
    }

    //Remove an element from a HashMap
    marks.remove("OOAD");

    //Print elements of a HashMap
    for (subject,mark) in &marks{
        println!("You scored {}% in {}", mark, subject);
    }

    //Check if HashMap contains a key
    println!("Did you take C++: {}", marks.contains_key("C++"));
}
