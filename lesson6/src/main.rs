//

// 13.04.23


// Enums


fn main() {

    // We kind of defining our own data type
    enum IpAddType {
        V4,
        V6,
    }

    // Creating an instance for the enums

    let four = IpAddType::V4;
    let six = IpAddType::V6;

    // passing the values in functions.

    fn route(ip_kind: IpAddType) {
        // Something;
    }
    // Calling the function 
    route(IpAddType::V4);

    struct IpTracker {
        kind: IpAddType,
        address: String,
    }

    let add1 = IpTracker{
        kind:IpAddType::V4,
        address: String::from("102.3.4.5"),
    };

    // println!("{}", add1.address);

    // Same problem without struct

    enum IpAddType2 {
        V4(String),
        V6(String),
        Vtest(u8,u8,i8,i8), // This is also possible
    }

    let add2 = IpAddType2::V4(String::from("123.23.1.1"));
    let add3 = IpAddType2::V6(String:: from("3.4.5.3"));

    enum Message { // This is also we able to define the enum where it's not possible in struct. & for struct we have to create seperate struct for each.
        Quite,
        Move { x: i32, y:i32},
        Write(String),
        ChangeColor(i32,i32,i32),
    }


    // Implementation 

    impl Message {
        fn call(&self) {
            // println!("Something ");
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call();

    // Option 
    // Rust don't have NULL, but we can  use it using Option<T>

    enum Option<T> {
        None,
        Some(T),
    }

    // let some_number = Some(3);
    // let absent_num :Option<i32> = None;

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin:Coin) -> u8{
        match coin {
            Coin::Penny => 1,
            Coin::Nickel=> 5,
            Coin::Dime=> 10,
            Coin::Quarter=> 25,
        }
    }

    // let age: i32 = 55;

    // match age {
    //     1..=18 => println!("A kid"),
    //     18..=55 => println!("Okay!"),
    //     _ => println!("Else condition"),
    // }

    enum Day{
        Monday,
        Tuesday,
        Thursday, 
        Sat,
        Sun,
    }

    impl Day {
        fn is_weekend(&self) -> bool{
            match self {

                Day::Sat | Day :: Sun => true,
                _ => false,
            }

        }
    }

    let today:Day = Day::Sat;
    println!("{}", today.is_weekend());
    // match today {

    // }

    // If let


}


