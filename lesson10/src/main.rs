
// Finding the biggest element in the vector

// Generic is also available for structs and enums 

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    X: U,
    Y: T,
}

fn main() {

    let v1:Vec<i32> = vec![230,46,12,43,6, 43,56,34];
    let v2:Vec<i32> = vec![230,46,12,43,6, 43,56,34];
    let v3:Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    let mut big_num = v1[0];

    for i in v1 {
        if i > big_num {
            big_num = i;
        }
    }

    // println!(" Biggest number:- {}", big_num);
    // println!(" Biggest number:- {}", get_biggst_num(v2));// Here we can't send the char type,need to send only int, so we use Generic data type.
    // println!(" Biggest number:- {}", get_biggest_num2(v2));
    // println!(" Biggest number:- {}", get_biggest_num2(v3));

    

}

fn get_biggst_num(vec:Vec<i32>) -> i32 {
    let mut big_num = vec[0];

    for i in vec {
        if i > big_num {
            big_num = i;
        }
    }

    big_num
}

fn get_biggest_num2<T: PartialOrd + Copy> (x: Vec<T>) -> T {
    let mut big_num = x[0];
    for i in x{
        if i > big_num {
            big_num = i;
        }
    }
    big_num
}