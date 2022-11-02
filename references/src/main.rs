fn main() {

    let mut x:u32 = 10;
    println!("The value of x is: {}",x);

//There can only exist one mutable reference.
//To have mutable and immutable references, the mutable reference needs to be in a code block
    {
        let x_reference = &mut x;
        *x_reference +=1;

        println!("The value of x is now: {} ",x_reference);
    }

    let y = &x;
    println!("The value of x is now: {}",y);

    println!("The value of x is now: {}",x);


}
