fn main() {
    let marks = 78;

    match marks{
        0..=20 => println!("Your grade is E"),
        21..=50 => println!("Your grade is D"),
        51..=70 => println!("Your grade is C"),
        71..=80 => println!("Your grade is B"),
        81..=100 => println!("Your grade is A"),
        _ => println!("Invalid marks!")
    }
}
