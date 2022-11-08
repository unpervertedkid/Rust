extern crate rand;
use rand::Rng;

fn main() {
    let num = rand::thread_rng().gen_range(1, 11);
    println!("Random number: {}", num);

    let  random_boolean = rand::thread_rng().gen_weighted_bool(2);
    println!("Random Boolean: {}",random_boolean);
}
