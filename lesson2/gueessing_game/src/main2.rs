// #[warn(unused_imports)]

use std:: io;
use rand:: Rng;
use std::cmp::Ordering;

fn main() {
    // println!("Guess the number");

    // println!("Enter the number");

    // let mut guess = String::new();

    // io::stdin().read_line(&mut guess).expect("Failed to Read the line");

    // println!("You gussed: {guess}");
    // println!("You gussed: {}", guess);// Another method to write a variables inside the string


    // const NAME : &str= "Const NAME";

    // let name:&str = "Another name";
    // let mut muttable_name= "Muattable name";

    // let age : &str = "123";

    // let mut age: u32 = age.trim().parse() Result<u32, ParseIntError>.expect(msg:"Something went wrong");

    // println!("Age of {} is {}", Name, age);

    // Will be generating int from range for 1-101, 101 in not included.
    // let rand_num: i32 = rand::thread_rng().gen_range(1..101);
    // println!("Random Number is {rand_num}")

    // IF else Conditions

    // let age: i32 = 15;
    // if age >= 18 {
    //     println!("You are an adult now");
    // }
    // else if age <= 5 {
    //     println!("You are a Baby");
    // }
    // else {
    //     println!("Nice");
    // }

    // Match statement
    //      Like Switch cases in c++

    // let age: i32 = 123;
    // match age {
    //     1..=18 =>println!("You can't Vote"), // 18 is included
    //     21|22 => println!("You can!"),
    //     65..=i32 :: MAX =>println!("You are tooo old"),

    //     _ => println!("This is the else statement"),
    // };
        // Another method
    // let age: i32 = 123;
    // let voting_age: i32 = 18;
    // match age.cmp(&voting_age){
    //     Ordering::Less=> println!("You can't Vote"),
    //     Ordering::Greater => println!("You can Vote"),
    //     Ordering::Equal => println!("Yuuuup"),
    // }


    //  Arrays
    // let arr = [1,2,3,4,5,6,7];
    // println!("length of array is {}", arr.len());
    // println!("Array at index 2 : {}", arr[2]);


    // Loop
    //      While
    // let mut loop_index: usize = 0;
    
    // while (loop_index < arr.len()) {
    //     println!("{}",arr[loop_index]);
    //     loop_index += 1;
    // }

    //      For Loop
    // let arr = [1,2,3,4,5,6,7];
    // for val  in arr.iter() {
    //     println!(" {} ", val);
    // }


    // Tuples
    // Multiple data types of fix size.  

    // let my_tuple: (u8, f64, String) = (23, 12_000.00, "Aditya".to_string());
    // let (v1: u8, v2:f64, v3:String) = my_tuple;

    // println!(" {}", my_tuple.0);
    // println!(" {}", my_tuple.1);
    // println!(" {}", my_tuple.2);

    // String
     

}


// https://youtu.be/ygL_xcavzQ4?t=2485