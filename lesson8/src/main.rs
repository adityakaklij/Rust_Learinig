use std::collections::HashMap;
// Stored only same type.
fn main() {
    // Defining empty vector.
    let v : Vec<i32> = Vec :: new(); 
    let mut v1 = vec![1,2,3]; // Using macros vec!
    let mut v2 = Vec :: new();
    // let mut v3 : Vec<&String> = vec!("asd", "df");

    v1.push(1);
    v2.push("Adi");
    v2.push( "Mas");
    // println!("Reading vector:-  {}", &v2[0]);
    // let third :Option<&i32> = v1.get(1);
    // println!("{}", &third);

    let mut m1:Vec<i32> = Vec::new();
    m1.push(21);
    m1.push(22);
    m1.push(23);
    m1.push(24);
    // println!("{}", m1[0])
    // println!("{:?}", m1.get(0));

    // match v1.get(1) {
    //     Some(x) => println!("at the match statement {}", x),
    //     None => println!("None"),
    // };

    // for i in &m1{
    //     println!("{}", i + 23);
    // }

    // HashMap

    let blue = String::from("Blue");
    let red = String::from("Red");
    
    let mut score: HashMap<String, i32> = HashMap::new();
    score.insert(blue, 23);
    score.insert(String::from("Blue"), 3);
    let blue1 = String::from("Blue");

    println!("{:?}", score.get(&blue1));
    println!("{:?}", score.get("Blue"));

    // score.insert("Red", 12)


}

