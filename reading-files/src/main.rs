use std::fs::File;
use std::io::prelude::*;

fn main(){

    let mut file = File::open("info.txt").expect("Error opening file");

    let mut file_content = String::new();

    file.read_to_string(&mut file_content).expect("Error reading from file");

    println!("File content:\n{}",file_content);
}
