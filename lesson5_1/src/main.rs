struct User {
    name:String,
    email:String,
    age:i32,
    active:bool,
}

struct Rectangle {
    width:i32,
    height:i32,
}

impl Rectangle {
    fn rec_area(&self) -> i32 {
        self.width * self.height
    }
}


fn main() {
    let mut user1  = User{
        email:String::from("user@gmail.com"),
        name:String::from("UserName 1"),
        age:18,
        active:true,
    };
    user1.name =String::from("name1");
    // println!("From user 1:- {}", user1.name);

    let user2 = create_user(
        String::from("user@gmail.com"),
        String::from("user 2"),
    );
    // println!(" From user 2:- {}", user2.name);

    let user3 = User {
        email:String::from("user3@gmail.com"),
        name:String::from("User 3"),
        ..user1
    };

    // println!(" From user 3:- {}", user3.name);

    let rec = (23,34);
    // println!("Area of rec: {}", get_area(rec));


    let rect = Rectangle {
        width: 23,
        height: 42,
    };

    // println!("Area of rect: {}", rec_area(&rect));
    println!("Area of rect using impl is :- {} ", rect.rec_area());

}

// Impleted this using impl 
// fn rec_area(dim:&Rectangle) -> i32 {
//     dim.width * dim.height
// }

fn get_area(dimensions:(u32,u32))-> u32 {
    dimensions.0 * dimensions.1
}

fn create_user(email:String, name:String)-> User {
    User{
        email:email,
        name,
        age:18,
        active:true,
    }
}