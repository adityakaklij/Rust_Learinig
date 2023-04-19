use std:: io;
use rand:: Rng;
// use std::cmp::Ordering;


fn main() {
    
    println!("Guess the number");
    // Taking input string from the user

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    println!("Enter a number");
    let mut guess = String::new();

    io :: stdin().read_line(&mut guess).expect("Failed to read!");

    println!("You entered {guess}")
    
}