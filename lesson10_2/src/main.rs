fn main() {
    let mut x:i32= 34;

    {
        let y:i32 = 3;
        let x = &y;
    }
    // println!("{}", x); // x will not print as it'a out of scope now  

    let str1 = String::from("abcde");
    let str2 = String::from("xyz");

    let result = longest_str(str1.as_str(), str2.as_str());
    println!("{}", result);
    

}

fn longest_str<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}
