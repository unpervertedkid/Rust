struct Person{
    name:String,
    age:u8
}
impl ToString for Person{
    fn to_string(&self) -> String{
        return format!("My name is {} and I am {} years old.",self.name,self.age);
    }
}
fn main() {
    let me = Person{name:String::from("Brian Silah"), age:21};

    println!("{}",me.to_string());
}
