// Ownership, variables scope String


fn main() {
    let x = 3;
    let s1 = String::from("This is the string");

    let y = x; // Possible in a case of int as it's having definite length, not in string
    let s2 = &s1;

    let s3 = s1.clone();

    // println!("{}",x);
    // println!("{}", y);
    // println!("{}", s1);
    // println!("{}", s2);
    // println!("{}", s3);

    // println!("{:p}", &x);
    // println!("{:p}", &y);
    // println!("{:p}", &s1);
    // println!("{:p}", &s2);
    // println!("{:p}", &s3);


    let a1 = String::from("Boom Boom!");

    let a2 = get_the_string_len(&a1);

    // println!("The length of \n {} is :- {}", a1, a2);


    fn get_the_string_len(a3:&String) -> usize{
        a3.len()
    }


    // String slicing

    let b1 = &a1[0..=2];
    let b3 = &a1[0..len];

    println!("{}", b1);
}