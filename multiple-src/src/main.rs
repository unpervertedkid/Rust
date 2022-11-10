mod rectangle;

fn main() {
    let my_rectangle = rectangle::create_rectangle(30, 50);
    println!("The area of the rectangle is {} square pixels.", my_rectangle.area());
    println!("Is it a square? {}", my_rectangle.is_square());
    println!("Can it hold another rectangle? {}", my_rectangle.can_hold(&rectangle::create_rectangle(10, 20)));

    let my_square = rectangle::create_square(30);
    println!("The area of the square is {} square pixels.", my_square.area());
    println!("Is it a square? {}", my_square.is_square());
    println!("Can it hold another rectangle? {}", my_square.can_hold(&rectangle::create_rectangle(150, 20)));
}
