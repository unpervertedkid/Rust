enum Language{
    Java,
    Rust,
    Python
}
fn main() {
    let language:Language = Language::Rust;

    match language{
        Language::Java => println!("You the tough guy huh!"),
        Language::Rust => println!("You the fast guy huh!"),
        Language::Python => println!("You the easy guy huh!"),
    }
}
