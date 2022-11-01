fn main() {
    print_to_n(20);
}
//Function that does not return
fn print_to_n(num:u32){
    for n in 1..num{
        if is_even(n){
            println!("{} is even",n);
        }else{
            println!("{} is odd",n);
        }
    }
}
//Function that returns
fn is_even(num:u32) -> bool{
    return num % 2 == 0;
}
