fn main() {
    
    let mut x = 5;
    
    x = 23;

    const BIG_NUMBER:u32 = 60*60*24; // Miutes in 24 Hrs

    // println!("{}", BIG_NUMBER);

    // Shadowing

    let x = 5;
    let x = x + 4;
    // let x = 22;

    {
        let x = x + 10;
        // println!("Inside the scope {}", x);
    }

    // println!("{}", x);

    let guess: u32 = "23".parse().expect("Not a number!");
    // println!("{}", guess);


    // Tupess

    // Method 1
    let tup1: (i16, u16, f32) = (-100, 200, 1.2223);
    // println!("{}",tup1.0);
    

    // Method 2
    let tup2 = (2,54, 3240);
    let (p, q, r) = tup2;
    // println!("{}", p)


    for i in (1..4).rev(){
        println!("{}", i);
    }
}
