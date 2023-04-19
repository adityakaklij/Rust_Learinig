// Impl, Implementation
    // It's a methods for the structs

struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle {
    fn print_area(&self){
        println!("Area from the impl is {}", self.width * self.height);
    }
}

fn main() {

    let my_rec = Rectangle{
        width: 10,
        height:20
    };
    // Method 1
    // println!("Area is {}" ,my_rec.width * my_rec.height); 

    // Method with using Implementation

    my_rec.print_area();


    
}