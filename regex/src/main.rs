extern crate regex;
use regex::Regex;

fn main() {
    let re = Regex::new(r"^\d{10}$").unwrap();

    let number1 = "0713567598";
    let number2 = "867667843";

    println!("Is phone number1 valid? : {}", re.is_match(number1));
    println!("Is phone number2 valid? : {}", re.is_match(number2));
}
