struct Person{
    name:String,
    age:u8
}

trait HasVoiceBox{
    fn can_speak(&self) -> bool;

    fn speak(&self);
}

impl HasVoiceBox for Person{
    fn can_speak(&self) -> bool{
        return self.age >= 1;
    }

    fn speak(&self){
        println!("Hello, my name is {}",self.name);
    }
}
fn main() {
    let person = Person{
        name:String::from("Bob"),
        age:23
    };

    if person.can_speak(){
        person.speak();
    }
}
