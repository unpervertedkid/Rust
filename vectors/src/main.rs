fn main() {
    //Declaring vectors
    let mut multiples_of_five: Vec<i32> = Vec::new();

    for number in 1..51{
        if number % 5 == 0{
            //Adding elements to vectors
            multiples_of_five.push(number);
        }
    }

    //Printing elements of a vector manually
    for num in 0..multiples_of_five.len()-1{
        println!("{}",multiples_of_five[num]);
    }
    println!();

    //Removing an element from a vectors
    multiples_of_five.remove(2); //Removing 15

    println!();

    //Printing element of a vector using iterator
    for num in multiples_of_five.iter(){
        println!("{}",num);
    }


}
