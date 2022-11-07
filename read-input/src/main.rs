use std::io;

fn main(){
    let mut marks = String::new();
    let mut name = String::new();

    println!("Enter your name:");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");


    println!("Enter marks to get grade: ");

    io::stdin()
        .read_line(&mut marks)
        .expect("Failed to read marks");

    let m:u8 = marks.trim().parse().expect("Invalid marks!");

    get_grade(&m,&name);


}

fn get_grade(marks:&u8, name:&String){
    match marks{
        0..=20 => println!("{}your grade is E",name),
        21..=50 => println!("{}your grade is D",name),
        51..=70 => println!("{}your grade is C",name),
        71..=80 => println!("{}your grade is B",name),
        81..=100 => println!("{}your grade is A",name),
        _ => println!("Invalid marks!")
    }
}
