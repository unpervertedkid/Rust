fn main() {
    /* Replace a string*/
    {
        let my_string = String::from("Rust is great");

        println!("Before Replace: {}", my_string);

        println!("After Replace: {}",my_string.replace("great","fantastic"));
    }

    /*Lines */
    {
        let my_string = String::from("Today \nWas \nSunny");

        println!("Before lines:\n{} ", my_string);

        println!("Using lines():");
        for line in my_string.lines(){
            println!("[ {} ]",line);
        }
    }

    /*Splits */
    {
        let my_string = String::from("Learning+Rust+Is+Easy");

        println!("Before split: {}", my_string);

        let tokens:Vec<&str> = my_string.split("+").collect();

        //Accesing token at index
        println!("At index 2: {}",tokens[2]);
    }

    /* Trim */
    {
        let my_string = String::from("       My name is Brian Silah\n\r");

        println!("Before Trim: {}", my_string);

        println!("After Trim: {}", my_string.trim());
    }

    /* Getting characters at indices */
    {
        let my_string = String::from("Do you think Java's syntax is simpler than Rust?");

        println!("Original String: {}",my_string);

        match my_string.chars().nth(5){
            Some(c) => println!("Char At index 5: {}", c),
            None => println!("Could not find character at index 5")
        }
    }
}
