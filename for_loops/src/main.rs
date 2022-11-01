const MAX_NUMBER:i8 = 100;
fn main() {
    for i in 1..MAX_NUMBER{
        if i%5 == 0{
            println!("{}",i);
        }
    }
}
