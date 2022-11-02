struct Color{
    blue:u8,
    green:u8,
    red:u8
}
fn main() {

    let mut bg = Color { blue: 100, green: 200, red:100 };

    bg.green = 45;

    println!("{}, {}, {}",bg.red,bg.green,bg.blue);
}
