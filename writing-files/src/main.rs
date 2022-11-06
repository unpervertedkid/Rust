use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("output.txt")
        .expect("Cannot create file");

    file.write_all(b"This is a file written by a rustacean in rust")
        .expect("Cannot write to this file");
}
