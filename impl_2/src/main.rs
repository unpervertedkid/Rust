struct Car{
    name:String,
    speed:u32,
    year:u32
}
impl ToString for Car{
    fn to_string(&self) -> String{
        return format!("Name: {} \nYear of manufacture: {} \nTop Speed: {}",self.name,self.year,self.speed);
    }
}
fn main() {

    let my_car = Car{name:String::from("Audi"), speed: 230, year:2020};
    println!("{}",my_car.to_string());

}
