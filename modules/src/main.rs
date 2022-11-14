
mod car{

    fn print_speed(name:&u32){
        println!("Speed: {} miles/hour",name);
    }
    fn  print_name(name:&str){
        println!("Name: {}",name);
    }

    pub fn get_car_details(){
        let name = String::from("Mercedes");
        let speed:u32 = 230;

        print_name(&name);
        print_speed(&speed);
    }
}
fn main() {
    car::get_car_details();
}
