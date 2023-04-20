// 19.04.23
// Enums
#[derive(Debug)]
enum TPAddType {
    V4,
    V6,
}

struct TPAdd{
    kind:TPAddType,
    ip:String,
}

enum AddTypeWithData {
    V4(String),
    V6(String),
    Something(u8,u8,i16),
}

fn main() {

    let four = TPAddType::V4;
    let six = TPAddType::V6;

    let ip1 = TPAdd{
        kind:TPAddType:: V4,
        ip: String::from("127.0.0.1"),
    };

    // println!("four: {:?}", four);
    // println!("{:?}", ip1.kind);
    // println!("{}", ip1.ip);

    let ip2 = AddTypeWithData::V4(String::from("new address"));
    // println!("{:#}", ip2);


    // Option enum
    // Option enmus are for the "None" if we want to none value

    let a = Some(3);
    let b = Some("Adi"); // It's define as 
//  let b: <Option<&str> = Some ("Adi");
    let c : Option<i32>= None;

    // println!("a: {:?}", c);


    enum Coins {
        Penny,
        Nickel,
        Dime,
        Quarter,

    }


    fn value_in_cents(coin: Coins) -> u8 {
        match coin {
            Coins::Penny => 0,
            Coins::Nickel => 1,
            Coins::Dime => 2,
            Coins::Quarter => 3,
        }
    }

    let m11 = value_in_cents(Coins::Nickel);
    println!("m11: {:?}", m11);
}