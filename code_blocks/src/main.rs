fn main() {

    let x:u32 = 20;
    /*
    This is a code block
    It can access variables outside its scope
    But variables in it cannot be acccesed outside
    */
    {
    let y:u32 = 30;
    println!("x: {}  y: {}",x,y);
    }

    //This will not work as it cannot access x
    //println!("x: {}  y: {}",x,y);
}
