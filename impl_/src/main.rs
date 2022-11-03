struct Rectangle{
    width:u32,
    height:u32
}

impl Rectangle{
    fn print_rectangle(&self){
        println!("Rectangle: {} * {}",self.width,self.height);
    }

    fn is_square(&self) -> bool{
        return self.height == self.width;
    }

    fn area(&self) -> u32{
        return self.width * self.height;
    }
}
fn main() {
    let rectangle1:Rectangle = Rectangle{height:20,width:25};

    rectangle1.print_rectangle();

    println!("Rectangle is a square: {}",rectangle1.is_square());

    println!("Area of Rectangle: {}",rectangle1.area());
}
