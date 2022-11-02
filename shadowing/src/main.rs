fn main() {
    let mut x:u32 = 15;

    println!("x is {}",x);

    {
        //Changing x in a code block
        x = 15;
        println!("x is now {}",x);
    }

    let mut y:u32 = 30;
    println!("y is {}",y);

    {
        //Shadowing using the keyword let
        let y = 20;
        println!("y is now {}",y);

        let y = "A string";

        println!("y is now {}",y);

        let y = true;

        println!("y is now {}",y);
    }

    //Outside the code block the value of y has not changed
    println!("y is still {}",y);

    //Changing y Outside the code block
    y = 50;
    println!("y is now {}",y);
}
