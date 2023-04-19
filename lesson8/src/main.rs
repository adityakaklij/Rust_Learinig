
// Stored only same type.
fn main() {
    // Defining empty vector.
    let v : Vec<i32> = Vec :: new(); 
    let mut v1 = vec![1,2,3]; // Using macros vec!
    let mut v2 = Vec :: new();
    let mut v3 : Vec<&String> = vec!("asd", "df");

    v1.push(1);
    v2.push("Adi");
    v2.push( "Mas");
    println!("Reading vector:-  {}", &v2[0]);
    // let third :Option<&i32> = v1.get(1);
    // println!("{}", &third);
}

