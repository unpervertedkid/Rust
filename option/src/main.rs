fn main() {
    let name = String::from("Rustacean");

    println!("String: {}",name);
    print_char_at_index(&name, 1);
    print_char_at_index(&name, 9);

    println!();
    let person_1 = String::from("Brian");
    let person_2 = String::from("Ilhan");
    let person_3 = String::from("Domenic");

    print_occupation(&person_1);
    print_occupation(&person_2);
    print_occupation(&person_3);
}

fn print_char_at_index(name:&str,index:usize){

    println!("Character at index {}: {}", index, match name.chars().nth(index){
        Some(c) => c.to_string(),
        None => "No character at specified index".to_string()
    });
}

fn print_occupation(name:&str){
    println!("{}'s occupation: {}",name, match get_occupation(&name){
        Some(occupation) => occupation,
        None => "Unknown"
    });
}
fn get_occupation(name:&str) -> Option<&str>{

    match name {
        "Brian" => Some("Software Engineer"),
        "Ilhan" => Some("Public Relations Officer"),
        _ => None
    }
}