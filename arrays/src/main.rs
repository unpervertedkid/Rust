fn main() {
    let mut numbers = [1,2,3,4];

    let mut numbers_copy = [0;4];

    //Change element
    numbers[3] = 5;



    //Manual iteration
    for i in 0..numbers.len(){
        println!("{}",numbers[i]);
        numbers_copy[i] = numbers[i];
    }

    println!("A copy of number");
    //For each loop
    for n in numbers_copy.iter(){
        println!("{}",n);
    }


}
