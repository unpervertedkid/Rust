struct Color(u8,u8,u8);
fn main() {

    let mut red = Color(255,60,0);

    red.1 = 0;

    println!("Red is {},{},{}",red.0,red.1,red.2);
}
