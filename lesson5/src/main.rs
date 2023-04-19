// Struct
// Custome dataType
#[derive(Debug)]
struct User {
    isactive: bool,
    name: String,
    age: u32,
}
fn main() {


    let user1 = User{
        isactive: true,
        name: String::from("AdiKa"),
        age:23,
    };

    // println!("{}", user1.isactive);
    let user2 = User{
        isactive:true,
        ..user1
    };
    // println!("{}", user2.age);

    fn register_user(name: String, age: u32) -> User {
        User{
            isactive:true,
            name:name,
            age:age,
        }
    }


    let m = register_user(String::from("Karan"), 43);
    // println!("{}" ,m.name);

    struct Color(i32, u32,i16);
    let black = Color(0,0,0);

    // println!("{}", black);


    // Calculating the area of the rectangle useing tupel
    let rec1 = (12,23);
    // println!("Area of the rectangle is:- {}", rec_area1(rec1));
    
    fn rec_area1(dimensions: (u32, u32)) -> u32{
        dimensions.0 * dimensions.1
    }

    // Calculating the area of the rectangle useing struct

    struct Rectanle {
        width: u32,
        height: u32,
    };

    let rec2 = Rectanle{
        width: 23,
        height: 56,
    };

    // println!("The calculated area using struct is:- {}", {rec_area2(&rec2)});

    fn rec_area2(rectangle: &Rectanle) -> u32 {
        rectangle.width * rectangle.height
    }


    




}

