fn main() {
    let tuples1 = (1,2,3,"A string",3.5,("A nested tuple!", 30, 56));
    let tuple2 = ("Rustacean", 21, 1);
    let (name,age,years_of_experience) = tuple2;

    println!("{}",(tuples1.5).0);

    println!("Name: {}, \nAge: {} \nYears of experience: {}",name,age,years_of_experience);
}
