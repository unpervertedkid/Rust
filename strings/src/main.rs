fn main() {
    let mut my_string = String::from("Hello from this crustacean.");
    //Find if string is empty
    println!("Is String Empty? : {}",my_string.is_empty());

    //Find length
    println!("Length: {}",my_string.len());

    //Splitting a string at whitespaces
    for token in my_string.split_whitespace(){
        println!("{}",token);
    }

    //Check is a string contains another String
    println!("Does string contain 'crustacean'?: {}",my_string.contains("crustacean"));

    //Appending to string --> Pushing in rust??!!
    my_string.push_str(" We are learning about strings");

    println!("{}",my_string);
}
