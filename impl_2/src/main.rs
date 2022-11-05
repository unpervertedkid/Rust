struct Car{
    name:String,
    speed:u32,
    year:u32
}
impl Car{
    fn get_name(&self){
        println!("Name: {}", self.name);
    }

    fn get_speed(&self){
        println!("Speed: {}", self.speed );
    }

    fn get_year(&self){
        println!("Year of manufacture: {}", self.year);
    }
}
fn main() {
    let name = String::from("Audi");
    let my_car = Car{name:name,speed:300,year:2022};
    my_car.get_name();
    my_car.get_year();
    my_car.get_speed();
}
